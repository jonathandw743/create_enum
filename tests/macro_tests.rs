use bevy_reflect::Reflect;
use enum_dispatch::enum_dispatch;

use create_enum::create_enum;

// #[derive(Clone, Copy, Reflect, Debug, PartialEq, Eq, Hash)]
// pub enum Direction {
//     PosX,
//     PosY,
//     NegX,
//     NegY,
// }

// impl Direction {
//     pub fn increment(&self) -> Self {
//         match self {
//             Self::PosX => Self::PosY,
//             Self::PosY => Self::NegX,
//             Self::NegX => Self::NegY,
//             Self::NegY => Self::PosX,
//         }
//     }
//     pub fn decrement(&self) -> Self {
//         match self {
//             Self::PosX => Self::NegY,
//             Self::PosY => Self::PosX,
//             Self::NegX => Self::PosY,
//             Self::NegY => Self::NegX,
//         }
//     }
// }

// #[derive(Reflect, Debug, Copy, Clone)]
// pub enum Input {
//     KeyCode,
//     MouseButton,
//     GamepadButton,
//     ThresholdedGamepadAxis,
// }

// #[enum_dispatch]
// pub trait TechTrait {
//     fn create_sprite(&self);
//     fn increment_direction(&mut self) {}
//     fn decrement_direction(&mut self) {}
// }

// #[derive(Debug, Reflect, Clone)]
// pub struct BoosterTech {
//     pub direction: Direction,
// }

// impl TechTrait for BoosterTech {
//     fn create_sprite(&self) {}
//     fn increment_direction(&mut self) {
//         self.direction = self.direction.increment();
//     }
//     fn decrement_direction(&mut self) {
//         self.direction = self.direction.decrement();
//     }
// }

// #[derive(Debug, Reflect, Clone)]
// pub struct InputTech {
//     pub input: Input,
// }

// impl TechTrait for InputTech {
//     fn create_sprite(&self) {}
// }

// #[derive(Debug, Reflect, Clone)]
// pub struct HighTech;

// impl TechTrait for HighTech {
//     fn create_sprite(&self) {}
// }

// #[derive(Debug, Reflect, Clone)]
// pub struct BrakeTech;

// impl TechTrait for BrakeTech {
//     fn create_sprite(&self) {}
// }

// #[create_enum(TechTrait)]
// pub struct Tech {
//     booster: BoosterTech,
//     brake: BrakeTech,
//     high: HighTech,
//     input: InputTech,
// }

// #[test]
// fn it_works() {
//     assert!(true);
// }

#[enum_dispatch]
pub trait TT {
    fn foo(&self) -> i32;
}

#[derive(Debug, Reflect, Clone)]
pub struct A(i32);

impl TT for A {
    fn foo(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Reflect, Clone)]
pub struct B(i32);

impl TT for B {
    fn foo(&self) -> i32 {
        self.0
    }
}

#[create_enum(TT)]
pub struct T {
    a: A,
    b: B,
}