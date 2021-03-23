use super::*;

impl<'a> Ciboulette2PostgresBuilderState<'a> {
    fn is_needed_included(
        &self,
        other: &CibouletteResourceType<'a>,
    ) -> Option<CibouletteResponseRequiredType> {
        match self.query().include().contains(other) {
            true => Some(**self.expected_response_type()),
            false => None,
        }
    }

    fn is_needed_path(
        &self,
        other: &CibouletteResourceType<'a>,
    ) -> Option<CibouletteResponseRequiredType> {
        match self.path() {
            CiboulettePath::Type(x) | CiboulettePath::TypeId(x, _) => match x == &other {
                true => Some(CibouletteResponseRequiredType::Object),
                // false => None
                false => None,
            },
            CiboulettePath::TypeIdRelated(x, _, y) => {
                if x == &other {
                    Some(CibouletteResponseRequiredType::None)
                } else if y == &other {
                    Some(CibouletteResponseRequiredType::Object)
                } else {
                    self.check_if_rel_is_needed(other, x, y)
                }
            }
            CiboulettePath::TypeIdRelationship(x, _, y) => {
                if x == &other {
                    Some(CibouletteResponseRequiredType::None)
                } else if y == &other {
                    Some(CibouletteResponseRequiredType::Id)
                } else {
                    self.check_if_rel_is_needed(other, x, y)
                }
            }
        }
    }

    fn is_needed_main_type(
        &self,
        other: &CibouletteResourceType<'a>,
    ) -> Option<CibouletteResponseRequiredType> {
        match &other == self.main_type() {
            true => Some(CibouletteResponseRequiredType::Object),
            false => None,
        }
    }

    fn is_needed_sorting(
        &self,
        other: &CibouletteResourceType<'a>,
    ) -> Option<CibouletteResponseRequiredType> {
        match self.query().sorting_map().contains_key(other) {
            true => Some(CibouletteResponseRequiredType::None),
            false => None,
        }
    }

    pub(crate) fn is_needed_all_for_relationships(
        &self,
        other: &CibouletteResourceType<'a>,
    ) -> Option<CibouletteResponseRequiredType> {
        self.is_needed_main_type(other)
            .or_else(|| self.is_needed_sorting(other))
    }

    pub(crate) fn is_needed_all(
        &self,
        other: &CibouletteResourceType<'a>,
    ) -> Option<CibouletteResponseRequiredType> {
        self.is_needed_included(other)
            .or_else(|| self.is_needed_path(other))
            .or_else(|| self.is_needed_main_type(other))
            .or_else(|| self.is_needed_sorting(other))
    }
}
