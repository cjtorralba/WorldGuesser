CREATE TABLE IF NOT EXISTS user_ranks
(
    id serial references user_creds,
    rank    INTEGER DEFAULT 0,
    total_score INTEGER DEFAULT 0,
    num_guesses INTEGER DEFAULT 0
)