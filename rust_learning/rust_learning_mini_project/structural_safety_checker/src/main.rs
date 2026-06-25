enum Material {
    Steel (f32), 
    Concrete (f32) , 
    Unknown
}
struct StructuralElement {
    id : u32 ,
    material : Material ,
    current_load : f32 ,
}


impl StructuralElement {
    fn torture_enum (&self) -> Option<f32> {
     match self.material {
        Material::Unknown => None ,
        Material::Steel(max_load) => Some(max_load) ,
        Material::Concrete(max_load) => Some(max_load) 
        }
    }

//     fn safe_check (&self ) -> Option<f32> {

//         if let Some(max_load)=  self.torture_enum() {
//             if max_load > self.current_load { 
//                 None
//             }
//             else {
//                 Some(max_load - self.current_load)
//             }
//         }
//         else {
//             None
//         }
//     }
}


fn main() {
    let stack_array = [
        StructuralElement {
            id : 00,
            material : Material::Steel(6969.69),
            current_load : 6767.67,
        },
                
        StructuralElement {
            id : 01,
            material : Material::Concrete(6767.67),
            current_load : 6969.69,
        },        
        StructuralElement {
            id : 02,
            material : Material::Unknown,
            current_load : 6767.67,
        },
    ];
    
    for element in stack_array.iter() {
        if let Some(max_load) =  element.torture_enum() {
            if max_load > element.current_load { 
                let remaining = max_load - element.current_load;
                println!("Element {} is SAFE. Remaining capacity: {}", element.id ,remaining)
            }
            else {
                
                println!("Element {} is NOTSAFE. UHOH! STRUCTURAL FAILURE!!!!!!!!", element.id)
            }
        }
        else {
            println!("WARNING: Element {} failed or is unknown!", element.id)
        }
    }

    }
   

