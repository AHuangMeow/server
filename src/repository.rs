use crate::{errors::AppError, models::user::User, constants::COLLECTION_USERS};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Collection, Database,
};

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<User>(COLLECTION_USERS),
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        Ok(self.collection
            .find_one(doc! { "email": email })
            .await?)
    }

    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, AppError> {
        Ok(self.collection
            .find_one(doc! { "_id": id })
            .await?)
    }

    pub async fn create(&self, user: &User) -> Result<(), AppError> {
        self.collection.insert_one(user).await?;
        Ok(())
    }

    pub async fn update_email(&self, id: &ObjectId, new_email: &str) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "email": new_email, "updated_at": DateTime::now() } },
            )
            .await?;
        Ok(())
    }

    pub async fn update_username(
        &self,
        id: &ObjectId,
        new_username: &str,
    ) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "username": new_username, "updated_at": DateTime::now() } },
            )
            .await?;
        Ok(())
    }

    pub async fn update_password(
        &self,
        id: &ObjectId,
        password_hash: &str,
    ) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "password_hash": password_hash, "updated_at": DateTime::now() } },
            )
            .await?;
        Ok(())
    }
}
