CREATE TABLE comments (
    id SERIAL PRIMARY KEY ,
    article_id int NOT NULL REFERENCES articles (id),
    author_id int NOT NULL REFERENCES users (id),
    body TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX comments_article_id_idx ON comments (article_id);
CREATE INDEX comments_author_id_idx ON comments (author_id);

SELECT diesel_manage_updated_at('comments');