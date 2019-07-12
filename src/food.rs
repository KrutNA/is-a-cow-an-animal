use crate::animal::AnimalType;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlantType {
    Grass,
    Carrot,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FoodType {
    Vegetable(PlantType),
    Meat(AnimalType),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Food {
    name: FoodType,
    energy: u64,
    eaten: bool,
}

impl Food {
    pub fn new(name: FoodType, energy: u64) -> Food {
        Food { name, energy, eaten: false }
    }
    pub fn name(&self) -> FoodType {
        self.name
    }
    pub fn energy(&self) -> u64 {
        self.energy
    }
    pub fn upd(&mut self) {
        self.eaten = true;
    }
    pub fn as_mut(&mut self) -> &mut Food {
        self
    }
    pub fn is_eaten(&self) -> bool {
        self.eaten
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn init_meat_human() {
        let food = Food::new(FoodType::Meat(AnimalType::Human), 55);
        assert_eq!((food.name(), food.energy()), (FoodType::Meat(AnimalType::Human), 55));
    }
    #[test]
    fn init_meat_cow() {
        let food = Food::new(FoodType::Meat(AnimalType::Cow), 33);
        assert_eq!((food.name(), food.energy()), (FoodType::Meat(AnimalType::Cow), 33));
    }
    #[test]
    fn init_meat_rabbit() {
        let food = Food::new(FoodType::Meat(AnimalType::Rabbit), 666);
        assert_eq!((food.name(), food.energy()), (FoodType::Meat(AnimalType::Rabbit), 666));
    }
    #[test]
    fn init_vegetable_carrot() {
        let food = Food::new(FoodType::Vegetable(PlantType::Carrot), 7);
        assert_eq!((food.name(), food.energy()), (FoodType::Vegetable(PlantType::Carrot), 7));
    }
    #[test]
    fn init_vegetable_grass() {
        let food = Food::new(FoodType::Vegetable(PlantType::Grass), 1);
        assert_eq!((food.name(), food.energy()), (FoodType::Vegetable(PlantType::Grass), 1));
    }
}
