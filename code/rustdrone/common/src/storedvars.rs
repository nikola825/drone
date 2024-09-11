use zerocopy::{AsBytes, FromBytes, LittleEndian, Unaligned, F32, I16, U16};

#[derive(Unaligned, Default, Clone)]
#[repr(C)]
struct StoredVariable {
    data: [u8; 8],
}

impl StoredVariable {
    pub fn get_ref<T: Unaligned + FromBytes>(&self) -> &T {
        T::ref_from(&self.data[..size_of::<T>()]).unwrap()
    }

    pub fn get_value<T: Unaligned + FromBytes>(&self) -> T {
        T::read_from(&self.data[..size_of::<T>()]).unwrap()
    }

    pub fn set_value(&mut self, data: &[u8]) {
        assert!(data.len() == 8);
        self.data.copy_from_slice(data);
    }
}

macro_rules! count_variables {
    ($typ1:ident $name1:ident)=>{
        (1)
    };
    ($typ1:ident $name1:ident, $($typ:ident $name:ident),*)=>{
        ((1)+(count_variables!($($typ $name),*)))
    }
}

macro_rules! variable_indices {
    ($name1: ident, $($name:ident),*)=>{
        //struct StoredVarIndices {}

        mod stored_variable_indices {
            variable_indices!((0), $name1, $($name),*);
        }
    };
    ($index: expr, $name1: ident)=>{
        pub const $name1 : usize = $index;
    };
    ($index: expr, $name1: ident, $($name:ident),*)=>{
        pub const $name1 : usize = $index;
        variable_indices!(($index+(1)), $($name),*);
    };
}

macro_rules! define_stored_variables {
    ($($typeprim:ident $typ:ident $name:ident),*)=>{
        variable_indices!($($name),*);

        #[derive(Unaligned,Default,Clone)]
        #[repr(C)]
        pub struct StoredVariables {
            variables: [StoredVariable; count_variables!($($typ $name),*)]
        }

        impl StoredVariables {
            $(
                pub fn $name (&self) -> $typeprim {
                    self.variables[stored_variable_indices::$name].get_ref::< $typ<LittleEndian>> ().get()
                }
            )*

            pub fn set_variable(&mut self, index: usize, data: &[u8]) {
                self.variables[index].set_value(data);
            }
        }

        pub trait VariableSetter {
            async fn set_variable<T: Unaligned + AsBytes>(&mut self, index: usize, value: T);
        }

        pub struct VariableSetterClient {

        }

        impl VariableSetterClient {
            $(
                pub async fn $name(value:$typeprim, setter: &mut impl VariableSetter) {
                    let value:$typ<LittleEndian> = $typ::from(value);
                    setter.set_variable(stored_variable_indices::$name, value).await;
                }
            )*
        }
    };
}

define_stored_variables!(
    u16 U16 Thrust_Input,
    i16 I16 Yaw_Input,
    i16 I16 Pitch_Input,
    i16 I16 Roll_Input,
    f32 F32 Yaw_Kp,
    f32 F32 Yaw_Ki,
    f32 F32 Yaw_Kd,
    f32 F32 Pitch_Kp,
    f32 F32 Pitch_Ki,
    f32 F32 Pitch_Kd,
    f32 F32 Roll_Kp,
    f32 F32 Roll_Ki,
    f32 F32 Roll_Kd
);
