// struct Person<PetType: Animal + NotDangerous>{ // to restrict the values from the not danger
//                                 //animlas to filter using + notDangerous
//     first_name: String,
//     pet: PetType,
// }


struct Person<PetType, PetType2: Animal+Dangerous>where PetType: Animal+ NotDangerous{
    first_name: String,
    pet: PetType,
    pet2: PetType2,
}

#[allow(dead_code)]
trait Animal{
    fn make_sound(&self) ->();
}

trait NotDangerous{}
trait Dangerous{}

struct Dog{}
impl NotDangerous for Dog{}
impl Animal for Dog {
    fn make_sound(&self) ->(){
        println!("Dog bark!");
    }
}

#[allow(dead_code)]
struct Tiger{}
impl Dangerous for Tiger{}
impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Tiger roared!");
    }
}

#[allow(dead_code)]
struct Cat{}
impl NotDangerous for Cat{}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("cat meowed!");
    }
}

pub fn create_person() {
    let dog = Dog{};
    let tiger = Tiger{};
    let cat= Cat{};
    let new_person = Person{first_name:"poppy".to_string(),pet: cat, pet2: cat};
    new_person.pet.make_sound();
}