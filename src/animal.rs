use crate::food::*;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AnimalType {
    Human,
    Cow,
    Rabbit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Animal {
    name: AnimalType,
    is_alive: bool,
    energy: u64,
}


// fn init_food(animal_type: AnimalType) -> Vec<FoodType> {
//     match animal_type {
//         AnimalType::Human => 
//             vec![FoodType::Meat(AnimalType::Cow),
//                  FoodType::Meat(AnimalType::Human),
//                  FoodType::Meat(AnimalType::Rabbit),
//                  FoodType::Vegetable(PlantType::Carrot)],
//         AnimalType::Rabbit =>
//             vec![FoodType::Vegetable(PlantType::Carrot)],
//         AnimalType::Cow =>
//             vec![FoodType::Vegetable(PlantType::Grass)],
//     }
// }

#[allow(dead_code)]
impl Animal {
    
    pub fn new(animal_type: AnimalType) -> Animal {
        Animal { name: animal_type, is_alive: true, energy: 0 }
    }
    
    pub fn new_e(animal_type: AnimalType, energy: u64) -> Animal {
        Animal { name: animal_type, is_alive: true, energy: energy}
    }
    
    pub fn eat(&mut self, food: &mut Food) -> &mut Animal {
        self.is_alive(self.is_alive, self.name, food.name());
        match food.name() {
            FoodType::Meat(_) => match food.is_eaten() {
                true => panic!("{:?} can't eat {:?}, it's already eaten"),
                false => food.upd(),
            },
            FoodType::Vegetable(_) => (),
        };
        self.energy += match self.check(food) {
            true => {
                food.energy()
            },
            false => panic!("{:?} can't eat {:?}", self.name, food.name()),
        };
        self
    } 
    pub fn eat_v(&mut self, vec: &mut Vec<Food>, id: usize) -> &mut Animal {
        match vec.get(id) {
            Some(_) => Some(()),
            None => panic!("Can't found value at {} in {:?}", id, vec),
        };
        match vec[id].name() {
            FoodType::Vegetable(_) => self.eat(vec.get_mut(id).unwrap()),
            _ =>  self.eat(&mut vec.remove(id)),
        }
    }
    pub fn die(&mut self) -> Food {
        if !self.is_alive {
            panic!("{:?} is already dead!", self.name);
        }
        self.is_alive = false;
        Food::new(FoodType::Meat(self.name), self.get_energy())
    }
    
    fn is_alive(&self, is_alive: bool, animal: AnimalType, food: FoodType) -> bool {
        if !is_alive {
            panic!("{:?} is dead! It can't eat {:?}", animal, food);
        }
        true
    }
    fn get_energy(&mut self) -> u64 {
        let tmp = self.energy;
        self.energy = 0;
        tmp
    }
    fn check(&self, food: &Food) -> bool {
        match self.name {
            AnimalType::Human => match food.name() {
                FoodType::Meat(_) | FoodType::Vegetable(PlantType::Carrot) => true,
                _ => false,
            }
            AnimalType::Cow => match food.name() {
                FoodType::Vegetable(PlantType::Grass) => true,
                _ => false,
            }
            AnimalType::Rabbit => match food.name() {
                FoodType::Vegetable(PlantType::Carrot) => true,
                _ => false,
            }
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_eating() {
        let mut human = Animal::new(AnimalType::Human);
        human.eat(Food::new(FoodType::Meat(AnimalType::Human), 5).as_mut())
            .eat(Food::new(FoodType::Meat(AnimalType::Cow), 5).as_mut())
            .eat(Food::new(FoodType::Meat(AnimalType::Rabbit), 5).as_mut())
            .eat(Food::new(FoodType::Vegetable(PlantType::Carrot), 5).as_mut());
        assert_eq!((human.name, human.energy), (AnimalType::Human, 20));
    }
    #[should_panic]
    #[test]
    fn human_not_eating() {
        let mut human = Animal::new(AnimalType::Human);
        human.eat(Food::new(FoodType::Vegetable(PlantType::Grass), 5).as_mut());     
    }
    #[test]
    fn rabbit_eating() {
        let mut rabbit = Animal::new_e(AnimalType::Rabbit, 10);
        rabbit.eat(Food::new(FoodType::Vegetable(PlantType::Carrot), 5).as_mut());
        assert_eq!((rabbit.name, rabbit.energy), (AnimalType::Rabbit, 15));
    }
    #[should_panic]
    #[test]
    fn rabbit_not_eating() {
        let mut rabbit = Animal::new_e(AnimalType::Rabbit, 10);
        rabbit.eat(Food::new(FoodType::Vegetable(PlantType::Grass), 5).as_mut());     
    }
    #[test]
    fn cow_eating() {
        let mut cow = Animal::new(AnimalType::Cow);
        cow.eat(Food::new(FoodType::Vegetable(PlantType::Grass), 5).as_mut());
        assert_eq!((cow.name, cow.energy), (AnimalType::Cow, 5));
    }
    #[should_panic]
    #[test]
    fn cow_not_eating() {
        let mut cow = Animal::new(AnimalType::Cow);
        cow.eat(Food::new(FoodType::Vegetable(PlantType::Carrot), 5).as_mut());     
    }
    #[test]
    fn die() {
        let mut human = Animal::new_e(AnimalType::Human, 10);
        let food = human.die();
        assert_eq!((food.name(), food.energy()),
                   (FoodType::Meat(AnimalType::Human), 10));
    }
    #[should_panic]
    #[test]
    fn cant_double_die() {
        let mut human = Animal::new_e(AnimalType::Human, 10);
        human.die();
        human.die();
    }
    #[should_panic]
    #[test]
    fn dead_not_eating() {
        let mut human = Animal::new_e(AnimalType::Human, 10);
        human.die();
        human.eat(Food::new(FoodType::Meat(AnimalType::Human), 5).as_mut());
    }
    #[test]
    fn eat_vegetable() {
        let mut food = Food::new(FoodType::Vegetable(PlantType::Carrot), 5);
        let mut rabbit = Animal::new(AnimalType::Rabbit);
        rabbit.eat(&mut food)
            .eat(&mut food);
    }
    #[should_panic]
    #[test]
    fn eat_meat() {
        let mut food = Food::new(FoodType::Meat(AnimalType::Human), 5);
        let mut human = Animal::new(AnimalType::Human);
        human.eat(&mut food)
            .eat(&mut food);
    }
}
