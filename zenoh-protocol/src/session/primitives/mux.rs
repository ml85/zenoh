//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use super::Primitives;
use crate::core::{CongestionControl, PeerId, Reliability, ResKey, ZInt};
use crate::core::{QueryConsolidation, QueryTarget, SubInfo};
use crate::io::RBuf;
use crate::proto::{zmsg, DataInfo, Declaration, ReplyContext, ZenohMessage};
use crate::session::SessionEventHandler;
use async_std::sync::Arc;
use async_trait::async_trait;

pub struct Mux<T: SessionEventHandler + Send + Sync + ?Sized> {
    handler: Arc<T>,
}

impl<T: SessionEventHandler + Send + Sync + ?Sized> Mux<T> {
    pub fn new(handler: Arc<T>) -> Mux<T> {
        Mux { handler }
    }
}

#[allow(unused_must_use)] // TODO
#[async_trait]
impl<T: SessionEventHandler + Send + Sync + ?Sized> Primitives for Mux<T> {
    async fn resource(&self, rid: ZInt, reskey: &ResKey) {
        let mut decls = Vec::new();
        decls.push(Declaration::Resource {
            rid,
            key: reskey.clone(),
        });
        self.handler
            .handle_message(ZenohMessage::make_declare(decls, None))
            .await;
    }

    async fn forget_resource(&self, rid: ZInt) {
        let mut decls = Vec::new();
        decls.push(Declaration::ForgetResource { rid });
        self.handler
            .handle_message(ZenohMessage::make_declare(decls, None))
            .await;
    }

    async fn subscriber(&self, reskey: &ResKey, sub_info: &SubInfo) {
        let mut decls = Vec::new();
        decls.push(Declaration::Subscriber {
            key: reskey.clone(),
            info: sub_info.clone(),
        });
        self.handler
            .handle_message(ZenohMessage::make_declare(decls, None))
            .await;
    }

    async fn forget_subscriber(&self, reskey: &ResKey) {
        let mut decls = Vec::new();
        decls.push(Declaration::ForgetSubscriber {
            key: reskey.clone(),
        });
        self.handler
            .handle_message(ZenohMessage::make_declare(decls, None))
            .await;
    }

    async fn publisher(&self, reskey: &ResKey) {
        let mut decls = Vec::new();
        decls.push(Declaration::Publisher {
            key: reskey.clone(),
        });
        self.handler
            .handle_message(ZenohMessage::make_declare(decls, None))
            .await;
    }

    async fn forget_publisher(&self, reskey: &ResKey) {
        let mut decls = Vec::new();
        decls.push(Declaration::ForgetPublisher {
            key: reskey.clone(),
        });
        self.handler
            .handle_message(ZenohMessage::make_declare(decls, None))
            .await;
    }

    async fn queryable(&self, reskey: &ResKey) {
        let mut decls = Vec::new();
        decls.push(Declaration::Queryable {
            key: reskey.clone(),
        });
        self.handler
            .handle_message(ZenohMessage::make_declare(decls, None))
            .await;
    }

    async fn forget_queryable(&self, reskey: &ResKey) {
        let mut decls = Vec::new();
        decls.push(Declaration::ForgetQueryable {
            key: reskey.clone(),
        });
        self.handler
            .handle_message(ZenohMessage::make_declare(decls, None))
            .await;
    }

    async fn data(
        &self,
        reskey: &ResKey,
        payload: RBuf,
        reliability: Reliability,
        congestion_control: CongestionControl,
        data_info: Option<DataInfo>,
    ) {
        self.handler
            .handle_message(ZenohMessage::make_data(
                reskey.clone(),
                payload,
                reliability,
                congestion_control,
                data_info,
                None,
                None,
            ))
            .await;
    }

    async fn query(
        &self,
        reskey: &ResKey,
        predicate: &str,
        qid: ZInt,
        target: QueryTarget,
        consolidation: QueryConsolidation,
    ) {
        let target_opt = if target == QueryTarget::default() {
            None
        } else {
            Some(target)
        };
        self.handler
            .handle_message(ZenohMessage::make_query(
                reskey.clone(),
                predicate.to_string(),
                qid,
                target_opt,
                consolidation.clone(),
                None,
            ))
            .await;
    }

    async fn reply_data(
        &self,
        qid: ZInt,
        source_kind: ZInt,
        replier_id: PeerId,
        reskey: ResKey,
        data_info: Option<DataInfo>,
        payload: RBuf,
    ) {
        self.handler
            .handle_message(ZenohMessage::make_data(
                reskey,
                payload,
                zmsg::default_reliability::REPLY,
                zmsg::default_congestion_control::REPLY,
                data_info,
                Some(ReplyContext::make(qid, source_kind, Some(replier_id))),
                None,
            ))
            .await;
    }

    async fn reply_final(&self, qid: ZInt) {
        self.handler
            .handle_message(ZenohMessage::make_unit(
                zmsg::default_reliability::REPLY,
                zmsg::default_congestion_control::REPLY,
                Some(ReplyContext::make(qid, 0, None)),
                None,
            ))
            .await;
    }

    async fn pull(
        &self,
        is_final: bool,
        reskey: &ResKey,
        pull_id: ZInt,
        max_samples: &Option<ZInt>,
    ) {
        self.handler
            .handle_message(ZenohMessage::make_pull(
                is_final,
                reskey.clone(),
                pull_id,
                *max_samples,
                None,
            ))
            .await;
    }

    async fn close(&self) {
        self.handler.closing().await;
    }
}
