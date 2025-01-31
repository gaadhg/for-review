# for-review

The project is still in its early stages, but its already somewhat messy.

I tried following the DDD principles so I started modeling the domain in which I should enforce the "business" logic. Things have gotten quite messy, logic has been duplicated a bit and I feel like I'm not doing the going in the right direction so I'll lay down what i've been thinking here:

To avoid having to deal with database schemas, ORMs and having 2nd struct for representing the data inside the database I tried using `native_db` so I could just serialize the struct using serde.


Data consistency should be enforced through the aggregate root (Student) so I tried to make all modifications to `Schedule`, `Event`, or `Cardset` must go through `Student` to prevent invalid states (for ex. You cannot add an event with subject label which isn't in `Student`'s subjects) but I feel I've modelled the APIs wrong and there is method duplication across the struct impls.

Some of the data "rules" are:
##### Subjects act as labels
- A subject added in one place (e.g., schedule, events, or cardsets) must automatically be available as a label everywhere else.
The Student struct maintains this invariant using a Set (IndexSet<Subject>).


```
              +------------------------+
              |         Student        |
              |------------------------|
              | id: UUID               |
              | email: Email           |
              | password: Password     |
              | subjects: IndexSet     |
              | schedule: Schedule     |
              | events: Vec<Event>     |
              | cardsets: Vec<CardSet> |
              +------------------------+
                         |
        --------------------------------------
        |                 |                 |
+---------------+ +---------------+ +------------------------+
|   Schedule    | |    Event      | |        Cardset         |
|---------------| |---------------| |------------------------|
| data: [7]     | | id: UUID      | | id: UUID               |
| subjects      | | name: String  | | name: String           |
|               | | description   | | label: Subject         |
|               | | label:Subject | | cards: Vec<Flashcard>  |
|               | | timestamp:... | |                        |
+---------------+ +---------------+ +------------------------+
```