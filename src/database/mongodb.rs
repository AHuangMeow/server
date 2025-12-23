use crate::constants::COLLECTION_USERS;
use crate::errors::AppError;
use crate::models::user::User;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};

pub async fn init_mongodb(uri: &str, db_name: &str) -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("ActixAuth".into());
    let client = Client::with_options(client_options)?;
    Ok(client.database(db_name))
}

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
        Ok(self.collection.find_one(doc! { "email": email }).await?)
    }

    pub async fn find_by_id(&self, id: &ObjectId) -> Result<Option<User>, AppError> {
        Ok(self.collection.find_one(doc! { "_id": id }).await?)
    }

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        use mongodb::bson::doc;
        let mut cursor = self.collection.find(doc! {}).await?;
        let mut users = Vec::new();
        use futures::stream::TryStreamExt;
        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }
        Ok(users)
    }

    pub async fn delete_by_id(&self, id: &ObjectId) -> Result<(), AppError> {
        self.collection.delete_one(doc! { "_id": id }).await?;
        Ok(())
    }

    pub async fn set_admin(&self, id: &ObjectId, is_admin: bool) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "is_admin": is_admin } },
            )
            .await?;
        Ok(())
    }

    pub async fn create(&self, user: &User) -> Result<(), AppError> {
        self.collection.insert_one(user).await?;
        Ok(())
    }

    pub async fn update_email(&self, id: &ObjectId, new_email: &str) -> Result<(), AppError> {
        self.collection
            .update_one(doc! { "_id": id }, doc! { "$set": { "email": new_email } })
            .await?;
        Ok(())
    }

    pub async fn update_username(&self, id: &ObjectId, new_username: &str) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "username": new_username } },
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
                doc! { "$set": { "password_hash": password_hash } },
            )
            .await?;
        Ok(())
    }

    pub async fn update_token_version(
        &self,
        id: &ObjectId,
        token_version: i32,
    ) -> Result<(), AppError> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "token_version": token_version } },
            )
            .await?;
        Ok(())
    }
}
