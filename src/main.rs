struct EntityData {
    health: u32,
}

macro_rules! create_entity {
    ($struct_name:ident $($field_name:ident:$field_type:ty),*) => {
        struct $struct_name {
            data: EntityData,
            $($field_name: $field_type),*
        }

        impl Entity for $struct_name {
            fn get_data(&self) -> &EntityData {
                &self.data
            }
            fn get_data_mut(&mut self) -> &mut EntityData {
                &mut self.data
            }
        }
    };
}

create_entity!(Zombie zombification: i32, something_else: String);
impl Updateable for Zombie {
    fn update(&mut self) {
        println!(
            "Zombie with the health of {} is updating!",
            self.data.health
        );
        println!("It has custom fields, too!");
        println!("{}, {}", self.zombification, self.something_else);
    }
}

create_entity!(Skeleton);
impl Updateable for Skeleton {
    fn update(&mut self) {
        println!("Skeleton is updating!");
        self.shoot();
    }
}

trait Shooting: Entity {
    fn shoot(&mut self);
}

trait Updateable {
    fn update(&mut self);
}

impl Shooting for Skeleton {
    fn shoot(&mut self) {
        println!(
            "The skeleton is shooting! It has {} health.",
            self.get_data().health
        );
        self.get_data_mut().health -= 4;
        println!(
            "Shit, it shot itself. It now has {} health.",
            self.get_data().health
        );
    }
}

trait Entity: Updateable {
    fn get_data(&self) -> &EntityData;
    fn get_data_mut(&mut self) -> &mut EntityData;
}

struct World {
    entities: Vec<Box<dyn Entity>>,
}
impl World {
    fn update(&mut self) {
        for e in self.entities.iter_mut() {
            e.update();
        }
    }
}

fn main() {
    let mut world = World {
        entities: Vec::new(),
    };
    world.entities.push(Box::new(Zombie {
        data: EntityData { health: 20 },
        zombification: 234,
        something_else: "WOWOWO".to_string(),
    }));
    world.entities.push(Box::new(Skeleton {
        data: EntityData { health: 20 },
    }));
    world.update();
}
