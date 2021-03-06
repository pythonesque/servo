/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLImageElementBinding;
use dom::bindings::codegen::InheritTypes::{NodeCast, HTMLImageElementDerived};
use dom::bindings::codegen::InheritTypes::{ElementCast};
use dom::bindings::js::JS;
use dom::bindings::error::ErrorResult;
use dom::document::Document;
use dom::element::{Element, HTMLImageElementTypeId};
use dom::element::{AttributeHandlers, AfterSetAttrListener, BeforeRemoveAttrListener};
use dom::eventtarget::{EventTarget, NodeTargetTypeId};
use dom::htmlelement::HTMLElement;
use dom::node::{Node, ElementNodeTypeId, NodeHelpers, window_from_node};
use servo_util::geometry::to_px;
use layout_interface::{ContentBoxQuery, ContentBoxResponse};
use servo_net::image_cache_task;
use servo_util::url::parse_url;
use servo_util::str::DOMString;
use url::Url;

use serialize::{Encoder, Encodable};

#[deriving(Encodable)]
pub struct HTMLImageElement {
    htmlelement: HTMLElement,
    priv extra: Untraceable,
}

struct Untraceable {
    image: Option<Url>,
}

impl<S: Encoder> Encodable<S> for Untraceable {
    fn encode(&self, _s: &mut S) {
    }
}

impl HTMLImageElementDerived for EventTarget {
    fn is_htmlimageelement(&self) -> bool {
        match self.type_id {
            NodeTargetTypeId(ElementNodeTypeId(HTMLImageElementTypeId)) => true,
            _ => false
        }
    }
}

impl HTMLImageElement {
    pub fn new_inherited(localName: DOMString, document: JS<Document>) -> HTMLImageElement {
        HTMLImageElement {
            htmlelement: HTMLElement::new_inherited(HTMLImageElementTypeId, localName, document),
            extra: Untraceable {
                image: None,
            }
        }
    }

    pub fn new(localName: DOMString, document: &JS<Document>) -> JS<HTMLImageElement> {
        let element = HTMLImageElement::new_inherited(localName, document.clone());
        Node::reflect_node(~element, document, HTMLImageElementBinding::Wrap)
    }
}

impl HTMLImageElement {
    pub fn image<'a>(&'a self) -> &'a Option<Url> {
        &self.extra.image
    }

    /// Makes the local `image` member match the status of the `src` attribute and starts
    /// prefetching the image. This method must be called after `src` is changed.
    fn update_image(&mut self, value: Option<DOMString>, url: Option<Url>) {
        let elem = &mut self.htmlelement.element;
        let document = elem.node.owner_doc();
        let window = document.get().window.get();
        let image_cache = &window.image_cache_task;
        match value {
            None => {
                self.extra.image = None;
            }
            Some(src) => {
                let img_url = parse_url(src, url);
                self.extra.image = Some(img_url.clone());

                // inform the image cache to load this, but don't store a
                // handle.
                //
                // TODO (Issue #84): don't prefetch if we are within a
                // <noscript> tag.
                image_cache.send(image_cache_task::Prefetch(img_url));
            }
        }
    }

    pub fn Alt(&self) -> DOMString {
        ~""
    }

    pub fn SetAlt(&mut self, _alt: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Src(&self, _abstract_self: &JS<HTMLImageElement>) -> DOMString {
        ~""
    }

    pub fn SetSrc(&mut self, abstract_self: &mut JS<HTMLImageElement>, src: DOMString) -> ErrorResult {
        let mut element: JS<Element> = ElementCast::from(abstract_self);
        element.set_url_attribute("src", src);
        Ok(())
    }

    pub fn CrossOrigin(&self) -> DOMString {
        ~""
    }

    pub fn SetCrossOrigin(&mut self, _cross_origin: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn UseMap(&self) -> DOMString {
        ~""
    }

    pub fn SetUseMap(&mut self, _use_map: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn IsMap(&self) -> bool {
        false
    }

    pub fn SetIsMap(&self, _is_map: bool) -> ErrorResult {
        Ok(())
    }

    pub fn Width(&self, abstract_self: &JS<HTMLImageElement>) -> u32 {
        let node: JS<Node> = NodeCast::from(abstract_self);
        let window = window_from_node(&node);
        let page = window.get().page();
        let (chan, port) = channel();
        let addr = node.to_trusted_node_address();
        let ContentBoxResponse(rect) = page.query_layout(ContentBoxQuery(addr, chan), port);
        to_px(rect.size.width) as u32
    }

    pub fn SetWidth(&mut self, abstract_self: &JS<HTMLImageElement>, width: u32) -> ErrorResult {
        let mut elem: JS<Element> = ElementCast::from(abstract_self);
        elem.set_attr(~"width", width.to_str())
    }

    pub fn Height(&self, abstract_self: &JS<HTMLImageElement>) -> u32 {
        let node = &self.htmlelement.element.node;
        let doc = node.owner_doc();
        let page = doc.get().window.get().page();
        let (chan, port) = channel();
        let this_node: JS<Node> = NodeCast::from(abstract_self);
        let addr = this_node.to_trusted_node_address();
        let ContentBoxResponse(rect) = page.query_layout(ContentBoxQuery(addr, chan), port);
        to_px(rect.size.height) as u32
    }

    pub fn SetHeight(&mut self, abstract_self: &JS<HTMLImageElement>, height: u32) -> ErrorResult {
        let mut elem: JS<Element> = ElementCast::from(abstract_self);
        elem.set_attr(~"height", height.to_str())
    }

    pub fn NaturalWidth(&self) -> u32 {
        0
    }

    pub fn NaturalHeight(&self) -> u32 {
        0
    }

    pub fn Complete(&self) -> bool {
        false
    }

    pub fn Name(&self) -> DOMString {
        ~""
    }

    pub fn SetName(&mut self, _name: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Align(&self) -> DOMString {
        ~""
    }

    pub fn SetAlign(&mut self, _align: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Hspace(&self) -> u32 {
        0
    }

    pub fn SetHspace(&mut self, _hspace: u32) -> ErrorResult {
        Ok(())
    }

    pub fn Vspace(&self) -> u32 {
        0
    }

    pub fn SetVspace(&mut self, _vspace: u32) -> ErrorResult {
        Ok(())
    }

    pub fn LongDesc(&self) -> DOMString {
        ~""
    }

    pub fn SetLongDesc(&mut self, _longdesc: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Border(&self) -> DOMString {
        ~""
    }

    pub fn SetBorder(&mut self, _border: DOMString) -> ErrorResult {
        Ok(())
    }
}

impl AfterSetAttrListener for JS<HTMLImageElement> {
    fn AfterSetAttr(&mut self, name: DOMString, value: DOMString) {
        if "src" == name {
            let window = window_from_node(self);
            let url = Some(window.get().get_url());
            self.get_mut().update_image(Some(value), url);
        }
    }
}

impl BeforeRemoveAttrListener for JS<HTMLImageElement> {
    fn BeforeRemoveAttr(&mut self, name: DOMString) {
        if "src" == name {
            self.get_mut().update_image(None, None);
        }
    }
}
