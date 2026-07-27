use salvo::prelude::*;

pub struct StateInjector<T: Clone + Send + Sync + 'static>(pub T);

#[async_trait]
impl<T: Clone + Send + Sync + 'static> Handler for StateInjector<T> {
    async fn handle(
        &self,
        _req: &mut Request,
        depot: &mut Depot,
        _res: &mut Response,
        flow: &mut FlowCtrl,
    ) {
        depot.insert_typed(self.0.clone());
        flow.call_next(_req, depot, _res).await;
    }
}
