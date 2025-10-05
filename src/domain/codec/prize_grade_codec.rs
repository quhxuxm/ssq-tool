use std::{collections::HashMap, fmt};

use serde::{
    Deserializer, Serializer,
    de::{SeqAccess, Visitor},
};

use crate::domain::{OfficialPrizeGrade, OfficialPrizeGradeType};

pub fn serialize<S>(
    data: &HashMap<OfficialPrizeGradeType, OfficialPrizeGrade>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut prize_grades = vec![];
    for prize_grade in data.values() {
        prize_grades.push(prize_grade);
    }
    prize_grades.sort_by(|v1, v2| v1.prize_type.cmp(&v2.prize_type));

    let prize_grades_string =
        serde_json::to_string(&prize_grades).map_err(serde::ser::Error::custom)?;

    serializer.serialize_str(&prize_grades_string)
}

pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<HashMap<OfficialPrizeGradeType, OfficialPrizeGrade>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PrizeGradeVisitor;

    impl<'de> Visitor<'de> for PrizeGradeVisitor {
        type Value = HashMap<OfficialPrizeGradeType, OfficialPrizeGrade>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a non-empty map of prize grade objects")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut prize_grades = HashMap::new();
            while let Some(prize_grade) = seq.next_element::<OfficialPrizeGrade>()? {
                prize_grades.insert(
                    prize_grade.prize_type,
                    OfficialPrizeGrade {
                        prize_type: prize_grade.prize_type,
                        prize_type_number: prize_grade.prize_type_number,
                        prize_type_money: prize_grade.prize_type_money,
                    },
                );
            }
            Ok(prize_grades)
        }
    }

    deserializer.deserialize_seq(PrizeGradeVisitor)
}
