# prisma-gpt
Ask for a SQL query based on your Prisma schema using GPT. Run it on your prisma project and it will prompt GPT with your schema and question. The prompt is engineered to return a raw SQL query (not prisma query) and a explanation. 
Inspired by [Replacing a SQL analyst with 26 recursive GPT prompts](https://www.patterns.app/blog/2023/01/18/crunchbot-sql-analyst-gpt/)

## Usage
It expects a `OPENAI_API_KEY` env variable to be set.
 
On your project directory, where prisma folder is:
   

     $ prisma-gpt "your question"

 
It will return something like this:

    all eligible sellers ordered by rating
    -- This query will retrieve all sellers who have been rated (i.e. have a rating above 0) and order them by rating in descending order.
    -- We will be using a combination of JOINs and aggregations to get the results.
    SELECT u.username, MAX(r.rating)
    FROM users u
    JOIN seller_info si ON u.id = si.user_id
    JOIN review r ON si.id = r.seller_id
    WHERE r.rating > 0
    GROUP BY u.username
    ORDER BY MAX(r.rating) DESC

## Explanation
It prompts GPT text-davinci-003 with the following template:

    {schema}
    As a senior analyst, given the above schemas and data, write a detailed and correct Postgres sql query to answer the analytical question:
    {question}
    Comment the query with your logic.

Depending on your schema, it may cost a lot of tokens or maybe exceed the token limit for the model. Use it with caution, it is just a experiment and it is not optimized for real use cases.
