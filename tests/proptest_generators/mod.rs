/*use super::*;
use proptest::*;
use proptest::prelude::*;
use proptest::arbitrary::Arbitrary;

pub fn int_type_any() -> impl Strategy<Value = IntType> {
    use IntType::*;
    prop_oneof![
        Just(UInt8),
        Just(UInt16),
        Just(UInt32),
        Just(UInt64),
        Just(SInt8),
        Just(SInt16),
        Just(SInt32),
        Just(SInt64),
    ]
}*/

/*
pub fn int_value_of_type(int_type: IntType) -> impl Strategy<Value = IntValue> {
    use IntType::*;

}*/

// Task generate random enum
// (1) choose random type + value count
// (2) generate values
/*
prop_compose! {
    pub(crate) fn enum_def_any()(
        encoding_type in int_type_any()
    )(
        drop_eligible_indicator in any::<bool>()
    ) -> EnumDef {
        SingleVlanHeader {
            priority_code_point: priority_code_point,
            drop_eligible_indicator: drop_eligible_indicator,
            vlan_identifier: vlan_identifier,
            ether_type: ether_type
        }
    }
}*/