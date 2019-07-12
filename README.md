## Is Cow An Animal?
### Description of world:
* Everything has some amount of energy
* There are animals: cow, rabbit, human
* There is food:
  + *Vegetables*: Grass, Carrot
  + *Meat*: Human, Cow, Rabbit

### General rules:

* Animals can eat some food, their energy is raised by the food's energy amount
* Animals can be slaughtered, the result is meat ; the energy of the meat is the same as the energy of the animal
* There should be a way to handle a list of animals and the energy of each animal
* Slaughtered animals can't eat anymore nor be slaughtered again
* Meat can be eaten only once
* Whereas vegetable is innumerable and can be eaten more than once
* Animals' accepted food:
  + A *cow* only eats grass
  + A *rabbit* only eats carrots
  + A *human* eats carrots and **any** meat

**IMPORTANT**: Feeding a cow with something else than grass is a fatal error (same for feeding a human with grass...) when slaughtered:

* A *cow* becomes *meat of cow* 
* A *rabbit* becomes *meat of rabbit*
* A *human* becomes *meat of human*

### [Idea from here](http://rigaux.org/language-study/various/is-a-cow-an-animal/)
But with some changes, as you can see

### Building and tests

* Build: `$ cargo build`
* Tests: `$ cargo test`
  + 6 tests for [`food`](./src/food.rs) module
  + 11 tests for [`animal`](./src/animal.rs) module
