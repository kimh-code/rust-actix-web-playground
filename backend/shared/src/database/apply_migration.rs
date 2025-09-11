use std::error::Error;
use tokio::fs;
use sqlx::{Row,
            types::chrono,
            PgPool, Pool, Postgres, Transaction, Executor,
};
use chrono::{DateTime, Utc};
use async_graphql::SimpleObject;

use crate::{
    error::Error as AppError,
};

#[derive(Clone)]
pub struct MigrationManager {
    migrations_dir: String, // \migrations
}

impl MigrationManager {
    pub async fn new(migrations_dir: String) -> Result<Self, sqlx::Error> {
        Ok(Self { migrations_dir })
    }

    pub async fn ensure_migration_table(pool: &PgPool) -> Result<(), AppError> {
        println!("마이그레이션 추적 테이블 확인 중...");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version BIGINT PRIMARY KEY,
                applied_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            );"
        )
        .execute(pool)
        .await?;

        println!("schema_migrations 테이블 준비 완료");

        Ok(())
    }

    pub async fn get_applied_migrations(pool: &PgPool) -> Result<Vec<i64>, AppError> {
        let versions: Vec<i64> = sqlx::query_scalar(
            "SELECT version FROM schema_migrations ORDER BY version"
        )
        .fetch_all(pool)
        .await?;

        println!("실행된 마이그레이션: {:?}", versions);
        Ok(versions)
    }

    pub async fn run_migration(pool: &PgPool , version: i64, sql: &str) -> Result<(), AppError> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = $1)"
        )
        .bind(version)
        .fetch_one(pool)
        .await?;

        if exists {
            println!("마이그레이션 {} 이미 실행됨", version);
            return Ok(());
        }

        println!("마이그레이션 {} 실행 중...", version);

        let mut tx: Transaction<Postgres> = pool.begin().await?;

        sqlx::query(sql)
            .execute(&mut *tx)
            .await?;
        
        sqlx::query(
            "INSERT INTO schema_migrations (version) VALUES ($1)"
        )
        .bind(version)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        println!("마이그레이션 {} 완료", version);
        Ok(())
    }

pub async fn extract_up_migration_files(&self) -> Result<Vec<String>, AppError> {
    println!("마이그레이션 디렉토리: {}", self.migrations_dir);
    println!("디렉토리 존재 여부: {}", std::path::Path::new(&self.migrations_dir).exists());
    
    let mut up_files = Vec::new();
    
    match tokio::fs::read_dir(&self.migrations_dir).await {
        Ok(mut entries) => {
            println!("디렉토리 읽기 성공, 파일 스캔 시작...");
            
            while let Some(entry) = entries.next_entry().await? {
                let file_type = entry.file_type().await?;
                let file_name = entry.file_name();
                let file_path = entry.path();
                
                println!("발견된 항목: {:?} (파일: {})", file_path, file_type.is_file());
                
                if file_type.is_file() {
                    if let Some(name_str) = file_name.to_str() {
                        println!("파일명: {}", name_str);
                        
                        if name_str.ends_with(".up.sql") {
                            println!(".up.sql 파일 발견: {}", name_str);
                            up_files.push(name_str.to_string());
                        } else {
                            println!(".up.sql 패턴 불일치: {}", name_str);
                        }
                    } else {
                        println!("파일명을 문자열로 변환 실패: {:?}", file_name);
                    }
                }
            }
        }
        Err(e) => {
            println!("디렉토리 읽기 실패: {}", e);
            return Err(e.into());
        }
    }
    
    up_files.sort();
    println!("최종 발견된 .up.sql 파일들: {:?}", up_files);
    println!("총 {} 개의 .up.sql 파일 발견", up_files.len());
    
    Ok(up_files)
}

    pub async fn extract_down_migration_files(&self) -> Result<Vec<String>, AppError> {
        let mut down_files = Vec::new();

        let mut entries = tokio::fs::read_dir(&self.migrations_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file_type = entry.file_type().await?;
            if file_type.is_file() {
                let down_file_name = entry.file_name();
                if let Some(name_str) = down_file_name.to_str() {
                    if name_str.ends_with(".down.sql") {
                       down_files.push(name_str.to_string());
                    }
                }
            }
        }

        down_files.sort();
        down_files.reverse();
        Ok(down_files)
    }

    fn extract_version_from_up_file(filename: &str) -> Option<i64> {
        let version_part = filename
            .trim_end_matches(".up.sql")
            .split('_')
            .next()?;

        version_part.parse().ok()
    }

    fn extract_version_from_down_file(filename: &str) -> Option<i64> {
        let version_part = filename
            .trim_end_matches(".down.sql")
            .split('_')
            .next()?;

        version_part.parse().ok()
    }

    fn find_down_file_for_up(&self,up_filename: &str) -> Option<String> {
        let down_filename = up_filename.replace(".up.sql", ".down.sql");

        let down_path = format!("{}/{}", self.migrations_dir, down_filename);
        if std::path::Path::new(&down_path).exists() {
            Some(down_filename)
        } else {
            None
        }
    }

    pub async fn find_pending_up_migrations(&self, pool: &PgPool) -> Result<Vec<(i64, String)>, AppError> {
        let up_files = Self::extract_up_migration_files(&self).await?;

        let applied_versions: Vec<i64> = Self::get_applied_migrations(pool).await?;

        println!("발견된 .up.sql 파일들:");
        for up_file in &up_files {
            if let Some(version) = Self::extract_version_from_up_file(up_file) {
                let down_file = self.find_down_file_for_up(up_file);
                println!("{} -> 버전: {}, down: {:?}", up_file, version, down_file);
                }
            }
        
        println!("\n DB에 적용된 마이그레이션:");
        for version in &applied_versions {
            println!("{}", version);
        }
     
        let mut pending = Vec::new();

        println!("파일 시스템의 마이그레이션:");
        for up_file in up_files {
            if let Some(version) = Self::extract_version_from_up_file(&up_file) {
                if !applied_versions.contains(&version) {
                    pending.push((version, up_file));
                }
            }
        }

        println!("\n 실행 대기 중인 .up.sql 마이그레이션:");
        for (version, file) in &pending {
            println!("{} ({})", version, file);
        }

        Ok(pending)
    }

    pub async fn run_pending_up_migrations(&self, pool: &PgPool) -> Result<(), AppError> {
        let pending = Self::find_pending_up_migrations(&self, pool).await?;

        if pending.is_empty() {
            println!("실행할 .up.sql 마이그레이션이 없습니다. 이미 최신 상태!");
            return Ok(());
        }

        for (version, up_filename) in pending {
            println!("UP 마이그레이션 실행 중: {} ({})", version, up_filename);

            let up_file_path = format!("{}/{}", self.migrations_dir, up_filename);
            let sql_content = fs::read_to_string(up_file_path).await?;

            let mut tx = pool.begin().await?;

            sqlx::raw_sql(&sql_content)
                .execute(&mut *tx)
                .await?;

            sqlx::query("INSERT INTO schema_migrations (version) VALUES ($1)")
                .bind(version)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;

            println!("UP 마이그레이션 {} 완료!", version);
        }

        println!("모든 UP 마이그레이션 실행 완료!");
        Ok(())
    }

    pub async fn rollback_to(&self, target_version: i64, pool: &PgPool) -> Result<(), AppError> {
        println!("목표: {}로 롤백", target_version);

        let versions_to_rollback: Vec<i64> = sqlx::query_scalar(
            "SELECT version FROM schema_migrations
            WHERE version > $1 ORDER BY version DESC"
        )
        .bind(target_version)
        .fetch_all(pool)
        .await?;

        if versions_to_rollback.is_empty() {
            println!("롤백할 마이그레이션이 없습니다.");
            return Ok(());
        }

        println!("롤백할 버전들: {:?}", versions_to_rollback);

        let down_files = self.extract_down_migration_files().await?;

        for version in versions_to_rollback {
            println!("버전 {} 롤백 중...", version);

            let down_filename = down_files.iter()
                .find(|filename|{
                    Self::extract_version_from_down_file(filename) == Some(version)
                });

            if let Some(down_filename) = down_filename {
                let down_file_path = format!("{}/{}", self.migrations_dir, down_filename);
                let sql_content = fs::read_to_string(down_file_path).await?;

                let mut tx = pool.begin().await?;

                sqlx::query(&sql_content)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("DELETE FROM schema_migrations WHERE version = $1")
                    .bind(version)
                    .execute(&mut *tx)
                    .await?;

                tx.commit().await?;

                println!("버전 {} 롤백 완료 ({})", version, down_filename);
            } else {
                println!("버전 {}의 down.sql 파일을 찾을 수 없습니다!", version);
                return Err(AppError::NotFound("down 파일 없음:".to_string()));
            }
        }

        println!("롤백 완료!");
        Ok(())
    }
    pub async fn migration_status(&self, pool: &PgPool) -> Result<(), AppError> {
        let rows = sqlx::query(
            r#"SELECT "version", applied_at FROM schema_migrations ORDER BY "version""#
        )
        .fetch_all(pool)
        .await?;

        println!("마이그레이션 상태:");
        println!("┌────────────────┬─────────────────────────────┐");
        println!("│ Version        │ Applied At                  │");
        println!("├────────────────┼─────────────────────────────┤");
        for row in rows {
            let version: i64 = row.get("version");
            let applied_at: chrono::DateTime<Utc> = row.get("applied_at");
            println!("| {:14} | {:27} |", version, applied_at.format("%Y-%m-%d %H:%M:%S UTC"));
        }

        println!("└────────────────┴─────────────────────────────┘");
        Ok(())
    }
}