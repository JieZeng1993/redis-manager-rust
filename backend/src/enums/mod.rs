///redis cluster type
#[derive(strum_macros::Display)]
pub enum ClusterType {
    STANDALONE,
    CLUSTER,
    SENTINEL,
}