use cmall_core::{EffectStatus, Role};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{error::AppError, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperateRole {
    pub code: String,
    pub name: String,
    pub description: String,
    pub status: EffectStatus,
}

impl AppState {
    pub async fn create_role(
        &self,
        input: &OperateRole,
        create_by: String,
    ) -> Result<Role, AppError> {
        let role = self.find_role_by_code(input.code.clone()).await?;
        if role.is_some() {
            return Err(AppError::RoleAlreadyExisted(input.code.clone()));
        }
        let role = sqlx::query_as(r#"
            insert into roles (code, name, description, status, create_by, update_by) values ($1, $2, $3, $4, $5, $6) returning id, code, name, status, description, create_time, create_by, update_time, update_by
        "#).bind(&input.code)
            .bind(&input.name)
            .bind(&input.description)
            .bind(&input.status)
            .bind(create_by.clone())
            .bind(create_by)
            .fetch_one(&self.pool)
            .await?;
        Ok(role)
    }

    pub async fn update_role(&self, id: i64, input: &OperateRole) -> Result<Role, AppError> {
        let role = self.find_role_by_id(id).await?;
        if role.is_none() {
            return Err(AppError::NotFound(format!("role id {}", id)));
        }
        let role = sqlx::query_as(r#"
            update roles set name = $1, description = $2, code = $3 where id = $4 returning id, code, name, description, create_time, create_by, status, update_time, update_by
        "#)
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.code)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(role)
    }
    pub async fn delete_role(&self, id: i64) -> Result<bool, AppError> {
        let role = self.find_role_by_id(id).await?;
        if role.is_none() {
            return Err(AppError::NotFound(format!("role id {}", id)));
        }
        let result = sqlx::query(
            r#"
            DELETE FROM roles WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("role id {}", id)));
        }
        Ok(true)
    }

    pub async fn find_role_by_condition(
        &self,
        code: Option<&str>,
        status: Option<EffectStatus>,
        page_num: i64,
        page_size: i64,
    ) -> Result<(Vec<Role>, i64), AppError> {
        // 需要根据分页信息查询
        let offset = (page_num - 1) * page_size;
        let roles = sqlx::query_as(
            r#"
            select id, code, name, description, create_time, create_by, status, update_time, update_by from roles 
            where (code = $1 or $1 is null) 
            and (status = $2 or $2 is null) 
            limit $3 offset $4
        "#,
        )
        .bind(code)
        .bind(status.clone())
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .unwrap();
        info!("find role by condition: {:?}", roles);

        // 获取满足条件的总记录数
        let total_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM roles
            WHERE (code = $1 OR $1 IS NULL)
            AND (status = $2 OR $2 IS NULL)
            "#,
        )
        .bind(code)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;
        Ok((roles, total_count))
    }

    pub async fn find_role_by_id(&self, id: i64) -> Result<Option<Role>, AppError> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            select id, code, name, description, create_time, create_by, status, update_time, update_by from roles where id = $1
        "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(role)
    }
    pub async fn find_role_by_code(&self, code: String) -> Result<Option<Role>, AppError> {
        let role = sqlx::query_as::<_, Role>(
            r#"
            select id, code, name, description, create_time, create_by, status, update_time, update_by from roles where code = $1
        "#,
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await?;
        Ok(role)
    }
}
