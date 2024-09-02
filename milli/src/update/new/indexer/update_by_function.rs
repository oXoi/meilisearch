use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::Indexer;
use crate::update::new::DocumentChange;
use crate::Result;

pub struct UpdateByFunction;

impl<'p> Indexer<'p> for UpdateByFunction {
    type Parameter = ();

    fn document_changes(
        self,
        _param: Self::Parameter,
    ) -> Result<impl ParallelIterator<Item = Result<Option<DocumentChange>>> + 'p> {
        todo!();
        Ok(vec![].into_par_iter())
    }
}
