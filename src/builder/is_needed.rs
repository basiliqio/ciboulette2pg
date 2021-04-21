use super::*;

impl<'store, 'request> Ciboulette2PostgresBuilderState<'store, 'request>
where
    'store: 'request,
{
    fn is_needed_included(
        &self,
        other: &CibouletteResourceType,
    ) -> Option<Ciboulette2PostgresResponseType> {
        match self.query().include().contains(other) {
            true => Some(self.expected_response_type()),
            false => None,
        }
    }

    fn is_needed_path(
        &self,
        other: &CibouletteResourceType,
    ) -> Option<Ciboulette2PostgresResponseType> {
        match self.path() {
            CiboulettePath::Type(x) | CiboulettePath::TypeId(x, _) => match x.as_ref() == other {
                true => Some(Ciboulette2PostgresResponseType::Object),
                // false => None
                false => None,
            },
            CiboulettePath::TypeIdRelated(x, _, y) => {
                if x.as_ref() == other {
                    Some(Ciboulette2PostgresResponseType::None)
                } else if y.related_type().as_ref() == other {
                    Some(Ciboulette2PostgresResponseType::Object)
                } else {
                    self.check_if_rel_is_needed(other, y)
                }
            }
            CiboulettePath::TypeIdRelationship(x, _, y) => {
                if x.as_ref() == other {
                    Some(Ciboulette2PostgresResponseType::None)
                } else if y.related_type().as_ref() == other {
                    Some(Ciboulette2PostgresResponseType::Id)
                } else {
                    self.check_if_rel_is_needed(other, y)
                }
            }
        }
    }

    fn is_needed_main_type(
        &self,
        other: &CibouletteResourceType,
    ) -> Option<Ciboulette2PostgresResponseType> {
        match other == self.main_type().as_ref() {
            true => Some(Ciboulette2PostgresResponseType::Object),
            false => None,
        }
    }

    pub(crate) fn is_needed_updating_relationships(
        &self,
        other: &CibouletteResourceType,
    ) -> Option<Ciboulette2PostgresResponseType> {
        match other == self.main_type().as_ref() {
            true => Some(Ciboulette2PostgresResponseType::Id),
            false => None,
        }
    }

    fn is_needed_sorting(
        &self,
        other: &CibouletteResourceType,
    ) -> Option<Ciboulette2PostgresResponseType> {
        match self.query().sorting_map().contains_key(other) {
            true => Some(Ciboulette2PostgresResponseType::None),
            false => None,
        }
    }

    pub(crate) fn is_needed_all_for_relationships(
        &self,
        other: &CibouletteResourceType,
    ) -> Option<Ciboulette2PostgresResponseType> {
        self.is_needed_main_type(other)
            .or_else(|| self.is_needed_sorting(other))
    }

    pub(crate) fn is_needed_all(
        &self,
        other: &CibouletteResourceType,
    ) -> Option<Ciboulette2PostgresResponseType> {
        self.is_needed_included(other)
            .or_else(|| self.is_needed_path(other))
            .or_else(|| self.is_needed_main_type(other))
            .or_else(|| self.is_needed_sorting(other))
    }
}
