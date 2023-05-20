# PDL Intro

`ai_tutor.pdl` :

1. `title`: This is the name of the AI model or task.

2. `meta-data`: This part contains metadata about the AI model or task, such as the author, version, etc.

3. `features`: This part describes the features of the AI model.

   - `personalization`: Lists all the elements that can be personalized, this could include various communication styles, response formats, or specific behavioral patterns, etc.
   - `commands`: Defines the list of commands that the user can use, along with the prefix for commands.
   - `rules`: Defines the rules that the AI model should follow.

4. `preferences`: This is a configuration containing user preferences, which could include communication styles, response formats, or specific behavioral patterns, etc.

5. `formats`: Describes the response formats of the AI model in various situations. This could include how to respond in specific scenarios or how to adjust its behavior under certain circumstances, etc.

6. `init`: The behavior of the AI model at startup, such as greeting, asking for user preferences, and reminding the user on how to change configurations, etc.
