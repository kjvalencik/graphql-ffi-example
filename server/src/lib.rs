use std::{str::FromStr, sync::RwLock};

use juniper::{
    graphql_object, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject,
    ID,
};

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

#[derive(Clone, GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(Clone, GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(Default)]
pub struct Context {
    humans: RwLock<Vec<Human>>,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    fn human(context: &Context, id: ID) -> FieldResult<Human> {
        let id = usize::from_str(&id)?;
        let lock = context.humans.read()?;
        let human = lock.get(id).ok_or_else(|| "Could not find human")?.clone();

        Ok(human)
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn createHuman(context: &Context, human: NewHuman) -> FieldResult<Human> {
        let mut lock = context.humans.write()?;
        let id = lock.len().to_string();
        let human = Human {
            id,
            name: human.name,
            appears_in: human.appears_in,
            home_planet: human.home_planet,
        };

        lock.push(human.clone());

        Ok(human)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use juniper::{graphql_value, DefaultScalarValue, ToInputValue, Variables};

    #[test]
    fn human() {
        let mut variables = Variables::new();

        variables.insert(
            "human".into(),
            NewHuman {
                name: "Luke Skywalker".into(),
                appears_in: vec![Episode::NewHope, Episode::Empire, Episode::Jedi],
                home_planet: "Tatooine".into(),
            }
            .to_input_value(),
        );

        let ctx = Context::default();
        let (res, _errors) = juniper::execute_sync(
            r#"
                mutation CreateHuman($human: NewHuman!) {
                    createHuman(human: $human) {
                        id
                    }
                }
            "#,
            None,
            &Schema::new(Query, Mutation, EmptySubscription::new()),
            &variables,
            &ctx,
        )
        .unwrap();

        assert_eq!(res, graphql_value!({ "createHuman": { "id": "0" } }));

        let mut variables: Variables<DefaultScalarValue> = Variables::new();

        variables.insert("id".into(), graphql_value!("0").to_input_value());

        let (res, _errors) = juniper::execute_sync(
            r#"
                query GetHuman($id: ID!) {
                    human(id: $id) {
                        id
                        name
                    }
                }
            "#,
            None,
            &Schema::new(Query, Mutation, EmptySubscription::new()),
            &variables,
            &ctx,
        )
        .unwrap();

        assert_eq!(
            res,
            graphql_value!({
                "human": {
                    "id": "0",
                    "name": "Luke Skywalker"
                }
            })
        );
    }
}
