use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::types::{
    HealthResponse, PublicKeyResponse, RedeemRequest, RedeemResponse, VerifyRequest,
    VerifyResponse, WithdrawRequest, WithdrawResponse,
};
use axum::extract::State;
use axum::Json;
use chrono::Utc;
use rsa::traits::PublicKeyParts;
use uuid::Uuid;

pub async fn health_check(State(state): State<AppState>) -> ApiResult<Json<HealthResponse>> {
    let db_status = sqlx::query("SELECT 1")
        .fetch_one(&state.db.pool)
        .await
        .map(|_| "ok")
        .unwrap_or("error");

    let redis_status = state
        .cache
        .health_check()
        .await
        .map(|_| "ok")
        .unwrap_or("error");

    Ok(Json(HealthResponse {
        status: "running".to_string(),
        database: db_status.to_string(),
        redis: redis_status.to_string(),
        timestamp: Utc::now().to_rfc3339(),
    }))
}

pub async fn get_public_key(State(state): State<AppState>) -> ApiResult<Json<PublicKeyResponse>> {
    let n = state.public_key.n();
    let e = state.public_key.e();

    Ok(Json(PublicKeyResponse {
        key_id: state.key_id().to_string(),
        institution_id: state.institution_id().to_string(),
        public_key_n: n.to_string(),
        public_key_e: e.to_string(),
        denominations: state.denominations().to_vec(),
        expires_at: None,
    }))
}

pub async fn withdraw(
    State(state): State<AppState>,
    Json(request): Json<WithdrawRequest>,
) -> ApiResult<Json<WithdrawResponse>> {
    if !state.is_valid_denomination(request.denomination) {
        return Err(ApiError::InvalidDenomination(request.denomination));
    }

    if request.blinded_tokens.is_empty() {
        return Err(ApiError::InvalidRequest("No tokens provided".to_string()));
    }

    let expected_count = request.amount.div_ceil(request.denomination);
    if request.blinded_tokens.len() != expected_count as usize {
        return Err(ApiError::InvalidRequest(format!(
            "Expected {} tokens for amount {} with denomination {}",
            expected_count, request.amount, request.denomination
        )));
    }

    let mut blind_signatures = Vec::new();

    for blinded_token in &request.blinded_tokens {
        if blinded_token.denomination != request.denomination {
            return Err(ApiError::InvalidRequest(
                "All tokens must have same denomination".to_string(),
            ));
        }

        let signature = state
            .institution
            .sign_blinded_token(blinded_token)
            .map_err(ApiError::Ecash)?;

        blind_signatures.push(signature);
    }

    let transaction_id = Uuid::new_v4().to_string();
    let expires_at = state.institution.expiry_time();

    let _ = state
        .db
        .log_transaction(crate::db::TransactionLog {
            transaction_type: "withdraw",
            amount: request.amount,
            denomination: request.denomination,
            token_count: request.blinded_tokens.len(),
            institution_id: state.institution_id(),
            key_id: state.key_id(),
            status: "success",
            error_message: None,
        })
        .await;

    Ok(Json(WithdrawResponse {
        blind_signatures,
        key_id: state.key_id().to_string(),
        expires_at: expires_at.to_rfc3339(),
        transaction_id,
    }))
}

pub async fn redeem(
    State(state): State<AppState>,
    Json(request): Json<RedeemRequest>,
) -> ApiResult<Json<RedeemResponse>> {
    if request.tokens.is_empty() {
        return Err(ApiError::InvalidRequest("No tokens provided".to_string()));
    }

    let mut accepted_count = 0;
    let mut total_amount = 0u64;
    let transaction_id = Uuid::new_v4().to_string();

    for token in &request.tokens {
        if token.is_expired() {
            return Err(ApiError::TokenExpired);
        }

        let serial_hex = token.serial_hex();

        if state.cache.is_token_spent(&serial_hex).await? {
            return Err(ApiError::TokenAlreadySpent);
        }

        if state.db.check_token_spent(&serial_hex).await? {
            return Err(ApiError::TokenAlreadySpent);
        }

        let is_valid = state
            .institution
            .verify_token(token)
            .map_err(ApiError::Ecash)?;
        if !is_valid {
            return Err(ApiError::InvalidSignature);
        }

        if !state
            .cache
            .check_and_mark_spent(&serial_hex, 86400 * 90)
            .await?
        {
            return Err(ApiError::TokenAlreadySpent);
        }

        let _ = state
            .db
            .mark_token_spent(
                token.serial_number.clone(),
                serial_hex,
                token.denomination,
                &token.currency,
                request.merchant_id.clone(),
            )
            .await?;

        accepted_count += 1;
        total_amount += token.denomination;
    }

    let _ = state
        .db
        .log_transaction(crate::db::TransactionLog {
            transaction_type: "redeem",
            amount: total_amount,
            denomination: request.tokens[0].denomination,
            token_count: request.tokens.len(),
            institution_id: state.institution_id(),
            key_id: state.key_id(),
            status: "success",
            error_message: None,
        })
        .await;

    Ok(Json(RedeemResponse {
        accepted_count,
        total_amount,
        transaction_id,
        timestamp: Utc::now().to_rfc3339(),
    }))
}

pub async fn verify(
    State(state): State<AppState>,
    Json(request): Json<VerifyRequest>,
) -> ApiResult<Json<VerifyResponse>> {
    let token = &request.token;
    let serial_hex = token.serial_hex();

    let expired = token.is_expired();
    let spent_redis = state
        .cache
        .is_token_spent(&serial_hex)
        .await
        .unwrap_or(false);
    let spent_db = state
        .db
        .check_token_spent(&serial_hex)
        .await
        .unwrap_or(false);
    let spent = spent_redis || spent_db;

    let valid = if expired || spent {
        false
    } else {
        state.institution.verify_token(token).unwrap_or(false)
    };

    let message = if expired {
        "Token has expired".to_string()
    } else if spent {
        "Token has already been spent".to_string()
    } else if !valid {
        "Invalid token signature".to_string()
    } else {
        "Token is valid".to_string()
    };

    Ok(Json(VerifyResponse {
        valid,
        expired,
        spent,
        message,
    }))
}
