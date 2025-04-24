use crate::types::answer::{Answer, AnswerId, NewAnswer};
use crate::types::question::{NewQuestion, Question, QuestionId};

use handle_errors::Error;

use sqlx::Row;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(_) => panic!("Couldn't establish DB connection!"),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, Error> {
        let limit_i32 = limit.map(|v| v as i32);

        match sqlx::query("select * from questions limit $1 offset $2")
            .bind(limit_i32)
            .bind(offset as i32)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn get_question(&self, question_id: i32) -> Result<Question, Error> {
	match sqlx::query(
	    "select * from questions where id = $1",
	)
	.bind(question_id)
	.map(|row: PgRow| Question {
	    id: QuestionId(row.get("id")),
	    title: row.get("title"),
	    content: row.get("content"),
	    tags: row.get("tags"),
	})
	.fetch_one(&self.connection)
	.await
	{
	    Ok(question) => Ok(question),
            //Err(sqlx::Error::RowNotFound) => {
            //    tracing::event!(tracing::Level::ERROR, "row not found");
	    //    Ok(Question::default())
            //},
	    Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
	    }
	}
    }

    pub async fn add_question(&self, question: NewQuestion) -> Result<Question, sqlx::Error> {
        sqlx::query(
	    "insert into questions (title, content, tags) values ($1, $2, $3) returning id, title, content, tags")
	.bind(question.title)
	.bind(question.content)
	.bind(question.tags)
	.map(|row: PgRow| Question {
	    id: QuestionId(row.get("id")),
	    title: row.get("title"),
	    content: row.get("content"),
	    tags: row.get("tags"),
	})
	.fetch_one(&self.connection)
	.await
    }

    pub async fn update_question(
        self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, sqlx::Error> {
        sqlx::query(
            "
	    update questions set title = $1, content = $2, tags = $3 where id = $4 
	    returning id, title, content, tags
	",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
    }

    pub async fn delete_question(self, question_id: i32) -> Result<bool, sqlx::Error> {
        match sqlx::query(
            "
	    delete from questions where id = $1
	",
        )
        .bind(question_id)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub async fn add_answer(&self, answer: NewAnswer) -> Result<Answer, Error> {
        match sqlx::query(
            "
	    insert into answer (content, question_id) values ($1, $2)
	",
        )
        .bind(answer.content)
        .bind(answer.question_id.0)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("question_id")),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}
