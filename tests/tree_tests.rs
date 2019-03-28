//! Tree module tests
//!

use libxml::parser::Parser;
use libxml::tree::{Document, Namespace, Node, NodeType};

#[test]
/// Root node and first child of root node are different
/// (There is a tiny chance this might fail for a correct program)
fn child_of_root_has_different_hash() {
  let parser = Parser::default();
  {
    let doc_result = parser.parse_file("tests/resources/file01.xml");
    assert!(doc_result.is_ok());
    let doc = doc_result.unwrap();
    let root = doc.get_root_element().unwrap();
    assert!(!root.is_text_node());
    if let Some(child) = root.get_first_child() {
      assert!(root != child);
    } else {
      assert!(false); //test failed - child doesn't exist
    }
    // same check with last child
    if let Some(child) = root.get_last_child() {
      assert!(root != child);
    } else {
      assert!(false); //test failed - child doesn't exist
    }
  }
}

#[test]
/// Siblings basic unit tests
fn node_sibling_accessors() {
  let mut doc = Document::new().unwrap();
  let hello_element_result = Node::new("hello", None, &doc);
  assert!(hello_element_result.is_ok());
  let mut hello_element = hello_element_result.unwrap();
  doc.set_root_element(&hello_element);

  let mut new_sibling = Node::new("sibling", None, &doc).unwrap();
  assert!(hello_element.add_prev_sibling(&mut new_sibling).is_ok());
}

#[test]
fn node_children_accessors() {
  // Setup
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let root = doc.get_root_element().unwrap();

  // Tests
  let root_children = root.get_child_nodes();
  assert_eq!(root_children.len(), 5, "file01 root has five child nodes");
  let mut element_children = root.get_child_elements();
  assert_eq!(
    element_children.len(),
    2,
    "file01 root has two child elements"
  );
  assert_eq!(element_children.pop().unwrap().get_name(), "child");
  assert_eq!(element_children.pop().unwrap().get_name(), "child");
  assert!(element_children.is_empty());
}

#[test]
fn node_attributes_accessor() {
  // Setup
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01.xml");
  assert!(doc_result.is_ok());
  let doc = doc_result.unwrap();
  let root = doc.get_root_element().unwrap();
  let mut root_elements = root.get_child_elements();
  let child_opt = root_elements.first_mut();
  assert!(child_opt.is_some());
  let child = child_opt.unwrap();

  // All attributes
  let attributes = child.get_attributes();
  assert_eq!(attributes.len(), 1);
  assert_eq!(attributes.get("attribute"), Some(&"value".to_string()));

  // Get
  assert_eq!(child.get_attribute("attribute"), Some("value".to_string()));
  // Get as node
  let attr_node_opt = child.get_attribute_node("attribute");
  assert!(attr_node_opt.is_some());
  let attr_node = attr_node_opt.unwrap();
  assert_eq!(attr_node.get_name(), "attribute");
  assert_eq!(attr_node.get_type(), Some(NodeType::AttributeNode));

  // Set
  assert!(child.set_attribute("attribute", "setter_value").is_ok());
  assert_eq!(
    child.get_attribute("attribute"),
    Some("setter_value".to_string())
  );
  // Remove
  assert!(child.remove_attribute("attribute").is_ok());
  assert_eq!(child.get_attribute("attribute"), None);
  // Recount
  let attributes = child.get_attributes();
  assert_eq!(attributes.len(), 0);
}

#[test]
fn attribute_namespace_accessors() {
  let mut doc = Document::new().unwrap();
  let element_result = Node::new("example", None, &doc);
  assert!(element_result.is_ok());

  let mut element = element_result.unwrap();
  doc.set_root_element(&element);

  let ns_result = Namespace::new(
    "myxml",
    "http://www.w3.org/XML/1998/namespace",
    &mut element,
  );
  assert!(ns_result.is_ok());
  let ns = ns_result.unwrap();
  assert!(element.set_attribute_ns("id", "testing", &ns).is_ok());

  let id_attr = element.get_attribute_ns("id", "http://www.w3.org/XML/1998/namespace");
  assert!(id_attr.is_some());
  assert_eq!(id_attr.unwrap(), "testing");

  let id_regular = element.get_attribute("id");
  assert!(id_regular.is_some());
  assert_eq!(id_regular.unwrap(), "testing");

  let id_false_ns = element.get_attribute_ns("id", "http://www.foobar.org");
  assert!(id_false_ns.is_none());
  let fb_ns_result = Namespace::new("fb", "http://www.foobar.org", &mut element);
  assert!(fb_ns_result.is_ok());
  let fb_ns = fb_ns_result.unwrap();
  assert!(element.set_attribute_ns("fb", "fb", &fb_ns).is_ok());

  let ns_prefix = element.lookup_namespace_prefix("http://www.w3.org/XML/1998/namespace");
  assert_eq!(ns_prefix, Some("xml".to_string())); // system ns has the global prefix when doing global lookup
  let fb_prefix = element.lookup_namespace_prefix("http://www.foobar.org");
  assert_eq!(fb_prefix, Some("fb".to_string())); // system ns has the global prefix when doing global lookup

  let ns_uri = element.lookup_namespace_uri("myxml");
  assert_eq!(
    ns_uri,
    Some("http://www.w3.org/XML/1998/namespace".to_string())
  ); // system ns has the global uri when doing global lookup
  let fb_uri = element.lookup_namespace_uri("fb");
  assert_eq!(fb_uri, Some("http://www.foobar.org".to_string())); // system ns has the global prefix when doing global lookup
}

#[test]
fn node_can_unbind() {
  let mut doc = Document::new().unwrap();
  let element_result = Node::new("example", None, &doc);
  assert!(element_result.is_ok());

  let mut element = element_result.unwrap();
  doc.set_root_element(&element);

  let mut first_child = Node::new("first", None, &doc).unwrap();
  let mut second_child = Node::new("second", None, &doc).unwrap();
  let mut third_child = Node::new("third", None, &doc).unwrap();

  assert!(element.add_child(&mut first_child).is_ok());
  assert!(element.add_child(&mut second_child).is_ok());
  assert!(element.add_child(&mut third_child).is_ok());

  assert_eq!(element.get_child_nodes().len(), 3);
  first_child.unbind_node();
  assert_eq!(element.get_child_nodes().len(), 2);
  second_child.unlink_node();
  assert_eq!(element.get_child_nodes().len(), 1);
  third_child.unlink();
  assert_eq!(element.get_child_nodes().len(), 0);

  // Test reparenting via unlink
  let mut transfer = Node::new("transfer", None, &doc).unwrap();
  assert!(element.add_child(&mut transfer).is_ok());
  assert!(transfer.append_text("test text").is_ok());
  let mut receiver = Node::new("receiver", None, &doc).unwrap();
  assert!(element.add_child(&mut receiver).is_ok());
  assert_eq!(element.get_child_nodes().len(), 2);
  assert_eq!(transfer.get_child_nodes().len(), 1);
  assert_eq!(receiver.get_child_nodes().len(), 0);

  transfer.unlink();
  assert_eq!(element.get_child_nodes().len(), 1);
  assert_eq!(receiver.get_child_nodes().len(), 0);
  assert!(receiver.add_child(&mut transfer).is_ok());
  assert_eq!(receiver.get_child_nodes().len(), 1);
  assert_eq!(transfer.get_content(), "test text".to_owned());
  assert_eq!(transfer.get_parent(), Some(receiver));
}

fn flex_drop_fn_2(glyph_string: &str) -> String {
    let p = Parser::default();
    let doc = p.parse_string(r##"<?xml version="1.0" standalone="no"?> <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.0//EN" "http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd" > <svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%">
<defs >
<font horiz-adv-x="874" ><font-face
    font-family="Luxi Serif"
    units-per-em="2048"
    panose-1="2 2 7 3 7 0 0 0 0 4"
    ascent="2073"
    descent="-432"
    alphabetic="0" />
<missing-glyph horiz-adv-x="512" d="M51 0V1480H461V0H51ZM410 51V1429H102V51H410Z" />
<glyph unicode=" " glyph-name="space" horiz-adv-x="512" />
<c g1="one" g2="X" k="32" />
<c g1="one" g2="X" k="77" />
<c g1="one" g2="X" k="48" />
<c g1="one" g2="X" k="48" />
<c g1="one" g2="X" k="32" />
<c g1="two" g2="X" k="23" />
<c g1="two" g2="X" k="49" />
<c g1="two" g2="X" k="5" />
<c g1="three" g2="X" k="43" />
<c g1="three" g2="X" k="-31" />
<c g1="three" g2="X" k="23" />
<c g1="four" g2="X" k="11" />
<c g1="four" g2="X" k="-41" />
<c g1="four" g2="X" k="27" />
<c g1="five" g2="X" k="47" />
<c g1="five" g2="X" k="-12" />
<c g1="five" g2="X" k="28" />
<c g1="six" g2="X" k="46" />
<c g1="six" g2="X" k="-35" />
<c g1="six" g2="X" k="28" />
<c g1="seven" g2="X" k="162" />
<c g1="seven" g2="X" k="162" />
<c g1="seven" g2="X" k="11" />
<c g1="seven" g2="X" k="-9" />
<c g1="seven" g2="X" k="-5" />
<c g1="seven" g2="X" k="144" />
<c g1="seven" g2="X" k="24" />
<c g1="seven" g2="X" k="46" />
<c g1="seven" g2="X" k="-6" />
<c g1="seven" g2="X" k="23" />
<c g1="seven" g2="X" k="128" />
<c g1="eight" g2="X" k="46" />
<c g1="eight" g2="X" k="-33" />
<c g1="eight" g2="X" k="27" />
<c g1="nine" g2="X" k="54" />
<c g1="nine" g2="X" k="-27" />
<c g1="nine" g2="X" k="31" />
<c g1="A" g2="X" k="-42" />
<c g1="A" g2="X" k="30" />
<c g1="A" g2="X" k="-42" />
<c g1="A" g2="X" k="101" />
<c g1="A" g2="X" k="101" />
<c g1="A" g2="X" k="97" />
<c g1="A" g2="X" k="97" />
<c g1="A" g2="X" k="101" />
<c g1="A" g2="X" k="101" />
<c g1="A" g2="X" k="214" />
<c g1="A" g2="X" k="190" />
<c g1="A" g2="X" k="138" />
<c g1="A" g2="X" k="1" />
<c g1="A" g2="X" k="-1" />
<c g1="A" g2="X" k="50" />
<c g1="A" g2="X" k="35" />
<c g1="A" g2="X" k="40" />
<c g1="A" g2="X" k="11" />
<c g1="A" g2="X" k="41" />
<c g1="A" g2="X" k="35" />
<c g1="A" g2="X" k="47" />
<c g1="A" g2="X" k="28" />
<c g1="A" g2="X" k="143" />
<c g1="A" g2="X" k="117" />
<c g1="A" g2="X" k="143" />
<c g1="A" g2="X" k="105" />
<c g1="A" g2="X" k="97" />
<c g1="A" g2="X" k="101" />
<c g1="A" g2="X" k="63" />
<c g1="A" g2="X" k="72" />
<c g1="A" g2="X" k="198" />
<c g1="A" g2="X" k="221" />
<c g1="A" g2="X" k="101" />
<c g1="A" g2="X" k="101" />
<c g1="A" g2="X" k="101" />
<c g1="A" g2="X" k="101" />
<c g1="B" g2="X" k="30" />
<c g1="B" g2="X" k="21" />
<c g1="B" g2="X" k="43" />
<c g1="B" g2="X" k="49" />
<c g1="B" g2="X" k="44" />
<c g1="B" g2="X" k="30" />
<c g1="B" g2="X" k="30" />
<c g1="B" g2="X" k="21" />
<c g1="B" g2="X" k="30" />
<c g1="B" g2="X" k="15" />
<c g1="B" g2="X" k="30" />
<c g1="B" g2="X" k="18" />
<c g1="B" g2="X" k="30" />
<c g1="B" g2="X" k="30" />
<c g1="B" g2="X" k="21" />
<c g1="B" g2="X" k="21" />
<c g1="B" g2="X" k="21" />
<c g1="C" g2="X" k="-27" />
<c g1="C" g2="X" k="-5" />
<c g1="C" g2="X" k="-5" />
<c g1="C" g2="X" k="59" />
<c g1="C" g2="X" k="-27" />
<c g1="C" g2="X" k="-27" />
<c g1="C" g2="X" k="59" />
<c g1="C" g2="X" k="-27" />
<c g1="C" g2="X" k="-27" />
<c g1="C" g2="X" k="59" />
<c g1="D" g2="X" k="112" />
<c g1="D" g2="X" k="128" />
<c g1="D" g2="X" k="26" />
<c g1="D" g2="X" k="94" />
<c g1="D" g2="X" k="97" />
<c g1="D" g2="X" k="104" />
<c g1="D" g2="X" k="95" />
<c g1="D" g2="X" k="112" />
<c g1="D" g2="X" k="112" />
<c g1="D" g2="X" k="112" />
<c g1="D" g2="X" k="112" />
<c g1="D" g2="X" k="112" />
<c g1="D" g2="X" k="112" />
<c g1="F" g2="X" k="142" />
<c g1="F" g2="X" k="0" />
<c g1="F" g2="X" k="142" />
<c g1="F" g2="X" k="172" />
<c g1="F" g2="X" k="134" />
<c g1="F" g2="X" k="57" />
<c g1="F" g2="X" k="123" />
<c g1="F" g2="X" k="73" />
<c g1="F" g2="X" k="54" />
<c g1="F" g2="X" k="95" />
<c g1="F" g2="X" k="73" />
<c g1="F" g2="X" k="50" />
<c g1="F" g2="X" k="47" />
<c g1="F" g2="X" k="172" />
<c g1="F" g2="X" k="172" />
<c g1="F" g2="X" k="57" />
<c g1="F" g2="X" k="123" />
<c g1="F" g2="X" k="82" />
<c g1="F" g2="X" k="116" />
<c g1="F" g2="X" k="73" />
<c g1="F" g2="X" k="73" />
<c g1="F" g2="X" k="73" />
<c g1="F" g2="X" k="123" />
<c g1="F" g2="X" k="73" />
<c g1="F" g2="X" k="172" />
<c g1="F" g2="X" k="172" />
<c g1="F" g2="X" k="73" />
<c g1="F" g2="X" k="172" />
<c g1="F" g2="X" k="172" />
<c g1="G" g2="X" k="22" />
<c g1="G" g2="X" k="43" />
<c g1="G" g2="X" k="23" />
<c g1="G" g2="X" k="30" />
<c g1="G" g2="X" k="24" />
<c g1="G" g2="X" k="22" />
<c g1="G" g2="X" k="22" />
<c g1="G" g2="X" k="22" />
<c g1="G" g2="X" k="22" />
<c g1="G" g2="X" k="22" />
<c g1="G" g2="X" k="22" />
<c g1="G" g2="X" k="22" />
<c g1="J" g2="X" k="40" />
<c g1="J" g2="X" k="40" />
<c g1="J" g2="X" k="40" />
<c g1="J" g2="X" k="40" />
<c g1="K" g2="X" k="163" />
<c g1="K" g2="X" k="105" />
<c g1="K" g2="X" k="104" />
<c g1="K" g2="X" k="100" />
<c g1="K" g2="X" k="-21" />
<c g1="K" g2="X" k="2" />
<c g1="K" g2="X" k="3" />
<c g1="K" g2="X" k="42" />
<c g1="K" g2="X" k="44" />
<c g1="K" g2="X" k="28" />
<c g1="K" g2="X" k="165" />
<c g1="K" g2="X" k="100" />
<c g1="K" g2="X" k="3" />
<c g1="K" g2="X" k="3" />
<c g1="K" g2="X" k="44" />
<c g1="K" g2="X" k="44" />
<c g1="K" g2="X" k="28" />
<c g1="K" g2="X" k="3" />
<c g1="K" g2="X" k="93" />
<c g1="K" g2="X" k="100" />
<c g1="L" g2="X" k="-2" />
<c g1="L" g2="X" k="-56" />
<c g1="L" g2="X" k="14" />
<c g1="L" g2="X" k="13" />
<c g1="L" g2="X" k="14" />
<c g1="L" g2="X" k="-6" />
<c g1="L" g2="X" k="115" />
<c g1="L" g2="X" k="41" />
<c g1="L" g2="X" k="208" />
<c g1="L" g2="X" k="162" />
<c g1="L" g2="X" k="152" />
<c g1="L" g2="X" k="15" />
<c g1="L" g2="X" k="122" />
<c g1="L" g2="X" k="-56" />
<c g1="L" g2="X" k="-56" />
<c g1="L" g2="X" k="18" />
<c g1="L" g2="X" k="14" />
<c g1="L" g2="X" k="41" />
<c g1="L" g2="X" k="15" />
<c g1="L" g2="X" k="-56" />
<c g1="L" g2="X" k="14" />
<c g1="L" g2="X" k="259" />
<c g1="L" g2="X" k="275" />
<c g1="L" g2="X" k="-56" />
<c g1="L" g2="X" k="14" />
<c g1="L" g2="X" k="14" />
<c g1="L" g2="X" k="14" />
<c g1="N" g2="X" k="18" />
<c g1="N" g2="X" k="18" />
<c g1="N" g2="X" k="46" />
<c g1="N" g2="X" k="50" />
<c g1="N" g2="X" k="51" />
<c g1="N" g2="X" k="50" />
<c g1="N" g2="X" k="69" />
<c g1="N" g2="X" k="51" />
<c g1="N" g2="X" k="52" />
<c g1="N" g2="X" k="53" />
<c g1="N" g2="X" k="46" />
<c g1="N" g2="X" k="46" />
<c g1="N" g2="X" k="51" />
<c g1="N" g2="X" k="50" />
<c g1="N" g2="X" k="69" />
<c g1="N" g2="X" k="69" />
<c g1="N" g2="X" k="69" />
<c g1="N" g2="X" k="51" />
<c g1="N" g2="X" k="52" />
<c g1="N" g2="X" k="52" />
<c g1="N" g2="X" k="53" />
<c g1="N" g2="X" k="46" />
<c g1="N" g2="X" k="69" />
<c g1="N" g2="X" k="46" />
<c g1="N" g2="X" k="46" />
<c g1="N" g2="X" k="50" />
<c g1="O" g2="X" k="101" />
<c g1="O" g2="X" k="30" />
<c g1="O" g2="X" k="95" />
<c g1="O" g2="X" k="98" />
<c g1="O" g2="X" k="101" />
<c g1="O" g2="X" k="96" />
<c g1="O" g2="X" k="101" />
<c g1="O" g2="X" k="101" />
<c g1="O" g2="X" k="101" />
<c g1="O" g2="X" k="101" />
<c g1="P" g2="X" k="185" />
<c g1="P" g2="X" k="24" />
<c g1="P" g2="X" k="185" />
<c g1="P" g2="X" k="172" />
<c g1="P" g2="X" k="166" />
<c g1="P" g2="X" k="39" />
<c g1="P" g2="X" k="32" />
<c g1="P" g2="X" k="32" />
<c g1="P" g2="X" k="172" />
<c g1="P" g2="X" k="172" />
<c g1="P" g2="X" k="39" />
<c g1="P" g2="X" k="39" />
<c g1="P" g2="X" k="39" />
<c g1="P" g2="X" k="32" />
<c g1="P" g2="X" k="32" />
<c g1="P" g2="X" k="32" />
<c g1="P" g2="X" k="211" />
<c g1="P" g2="X" k="39" />
<c g1="P" g2="X" k="32" />
<c g1="P" g2="X" k="30" />
<c g1="P" g2="X" k="172" />
<c g1="R" g2="X" k="105" />
<c g1="R" g2="X" k="95" />
<c g1="R" g2="X" k="96" />
<c g1="R" g2="X" k="95" />
<c g1="R" g2="X" k="86" />
<c g1="R" g2="X" k="104" />
<c g1="R" g2="X" k="112" />
<c g1="R" g2="X" k="118" />
<c g1="R" g2="X" k="113" />
<c g1="R" g2="X" k="4" />
<c g1="R" g2="X" k="43" />
<c g1="R" g2="X" k="45" />
<c g1="R" g2="X" k="29" />
<c g1="R" g2="X" k="81" />
<c g1="R" g2="X" k="96" />
<c g1="R" g2="X" k="95" />
<c g1="R" g2="X" k="104" />
<c g1="R" g2="X" k="4" />
<c g1="R" g2="X" k="4" />
<c g1="R" g2="X" k="4" />
<c g1="R" g2="X" k="43" />
<c g1="R" g2="X" k="45" />
<c g1="R" g2="X" k="45" />
<c g1="R" g2="X" k="29" />
<c g1="R" g2="X" k="29" />
<c g1="R" g2="X" k="4" />
<c g1="R" g2="X" k="91" />
<c g1="R" g2="X" k="45" />
<c g1="R" g2="X" k="95" />
<c g1="S" g2="X" k="30" />
<c g1="S" g2="X" k="22" />
<c g1="S" g2="X" k="3" />
<c g1="S" g2="X" k="9" />
<c g1="S" g2="X" k="3" />
<c g1="S" g2="X" k="38" />
<c g1="S" g2="X" k="30" />
<c g1="S" g2="X" k="30" />
<c g1="S" g2="X" k="30" />
<c g1="S" g2="X" k="30" />
<c g1="T" g2="X" k="143" />
<c g1="T" g2="X" k="142" />
<c g1="T" g2="X" k="143" />
<c g1="T" g2="X" k="197" />
<c g1="T" g2="X" k="197" />
<c g1="T" g2="X" k="100" />
<c g1="T" g2="X" k="28" />
<c g1="T" g2="X" k="30" />
<c g1="T" g2="X" k="97" />
<c g1="T" g2="X" k="28" />
<c g1="T" g2="X" k="0" />
<c g1="T" g2="X" k="-51" />
<c g1="T" g2="X" k="-45" />
<c g1="T" g2="X" k="-50" />
<c g1="T" g2="X" k="175" />
<c g1="T" g2="X" k="195" />
<c g1="T" g2="X" k="191" />
<c g1="T" g2="X" k="183" />
<c g1="T" g2="X" k="38" />
<c g1="T" g2="X" k="95" />
<c g1="T" g2="X" k="192" />
<c g1="T" g2="X" k="131" />
<c g1="T" g2="X" k="137" />
<c g1="T" g2="X" k="192" />
<c g1="T" g2="X" k="208" />
<c g1="T" g2="X" k="214" />
<c g1="T" g2="X" k="208" />
<c g1="T" g2="X" k="100" />
<c g1="T" g2="X" k="100" />
<c g1="T" g2="X" k="28" />
<c g1="T" g2="X" k="100" />
<c g1="T" g2="X" k="29" />
<c g1="T" g2="X" k="175" />
<c g1="T" g2="X" k="151" />
<c g1="T" g2="X" k="201" />
<c g1="T" g2="X" k="100" />
<c g1="T" g2="X" k="100" />
<c g1="T" g2="X" k="28" />
<c g1="T" g2="X" k="25" />
<c g1="T" g2="X" k="230" />
<c g1="T" g2="X" k="100" />
<c g1="T" g2="X" k="100" />
<c g1="T" g2="X" k="28" />
<c g1="T" g2="X" k="28" />
<c g1="T" g2="X" k="28" />
<c g1="U" g2="X" k="33" />
<c g1="U" g2="X" k="27" />
<c g1="U" g2="X" k="92" />
<c g1="U" g2="X" k="44" />
<c g1="U" g2="X" k="44" />
<c g1="U" g2="X" k="32" />
<c g1="U" g2="X" k="44" />
<c g1="U" g2="X" k="92" />
<c g1="U" g2="X" k="92" />
<c g1="U" g2="X" k="92" />
<c g1="U" g2="X" k="92" />
<c g1="U" g2="X" k="92" />
<c g1="U" g2="X" k="92" />
<c g1="V" g2="X" k="189" />
<c g1="V" g2="X" k="91" />
<c g1="V" g2="X" k="187" />
<c g1="V" g2="X" k="142" />
<c g1="V" g2="X" k="138" />
<c g1="V" g2="X" k="216" />
<c g1="V" g2="X" k="94" />
<c g1="V" g2="X" k="97" />
<c g1="V" g2="X" k="95" />
<c g1="V" g2="X" k="31" />
<c g1="V" g2="X" k="-50" />
<c g1="V" g2="X" k="140" />
<c g1="V" g2="X" k="150" />
<c g1="V" g2="X" k="135" />
<c g1="V" g2="X" k="19" />
<c g1="V" g2="X" k="149" />
<c g1="V" g2="X" k="72" />
<c g1="V" g2="X" k="64" />
<c g1="V" g2="X" k="61" />
<c g1="V" g2="X" k="216" />
<c g1="V" g2="X" k="216" />
<c g1="V" g2="X" k="95" />
<c g1="V" g2="X" k="247" />
<c g1="V" g2="X" k="95" />
<c g1="V" g2="X" k="140" />
<c g1="V" g2="X" k="149" />
<c g1="V" g2="X" k="158" />
<c g1="V" g2="X" k="216" />
<c g1="V" g2="X" k="216" />
<c g1="V" g2="X" k="95" />
<c g1="V" g2="X" k="187" />
<c g1="V" g2="X" k="216" />
<c g1="V" g2="X" k="216" />
<c g1="V" g2="X" k="95" />
<c g1="V" g2="X" k="95" />
<c g1="V" g2="X" k="95" />
<c g1="W" g2="X" k="154" />
<c g1="W" g2="X" k="72" />
<c g1="W" g2="X" k="152" />
<c g1="W" g2="X" k="144" />
<c g1="W" g2="X" k="141" />
<c g1="W" g2="X" k="202" />
<c g1="W" g2="X" k="101" />
<c g1="W" g2="X" k="104" />
<c g1="W" g2="X" k="102" />
<c g1="W" g2="X" k="37" />
<c g1="W" g2="X" k="-44" />
<c g1="W" g2="X" k="142" />
<c g1="W" g2="X" k="135" />
<c g1="W" g2="X" k="133" />
<c g1="W" g2="X" k="25" />
<c g1="W" g2="X" k="136" />
<c g1="W" g2="X" k="76" />
<c g1="W" g2="X" k="66" />
<c g1="W" g2="X" k="63" />
<c g1="W" g2="X" k="202" />
<c g1="W" g2="X" k="202" />
<c g1="W" g2="X" k="102" />
<c g1="W" g2="X" k="205" />
<c g1="W" g2="X" k="97" />
<c g1="W" g2="X" k="142" />
<c g1="W" g2="X" k="131" />
<c g1="W" g2="X" k="138" />
<c g1="W" g2="X" k="202" />
<c g1="W" g2="X" k="202" />
<c g1="W" g2="X" k="102" />
<c g1="W" g2="X" k="167" />
<c g1="W" g2="X" k="202" />
<c g1="W" g2="X" k="202" />
<c g1="W" g2="X" k="102" />
<c g1="W" g2="X" k="102" />
<c g1="W" g2="X" k="102" />
<c g1="X" g2="X" k="103" />
<c g1="X" g2="X" k="95" />
<c g1="X" g2="X" k="94" />
<c g1="X" g2="X" k="94" />
<c g1="X" g2="X" k="16" />
<c g1="X" g2="X" k="55" />
<c g1="X" g2="X" k="57" />
<c g1="X" g2="X" k="41" />
<c g1="X" g2="X" k="124" />
<c g1="X" g2="X" k="94" />
<c g1="Y" g2="X" k="177" />
<c g1="Y" g2="X" k="164" />
<c g1="Y" g2="X" k="177" />
<c g1="Y" g2="X" k="182" />
<c g1="Y" g2="X" k="176" />
<c g1="Y" g2="X" k="143" />
<c g1="Y" g2="X" k="95" />
<c g1="Y" g2="X" k="98" />
<c g1="Y" g2="X" k="96" />
<c g1="Y" g2="X" k="32" />
<c g1="Y" g2="X" k="-49" />
<c g1="Y" g2="X" k="182" />
<c g1="Y" g2="X" k="192" />
<c g1="Y" g2="X" k="182" />
<c g1="Y" g2="X" k="19" />
<c g1="Y" g2="X" k="193" />
<c g1="Y" g2="X" k="95" />
<c g1="Y" g2="X" k="104" />
<c g1="Y" g2="X" k="101" />
<c g1="Y" g2="X" k="143" />
<c g1="Y" g2="X" k="143" />
<c g1="Y" g2="X" k="96" />
<c g1="Y" g2="X" k="143" />
<c g1="Y" g2="X" k="96" />
<c g1="Y" g2="X" k="182" />
<c g1="Y" g2="X" k="185" />
<c g1="Y" g2="X" k="224" />
<c g1="Y" g2="X" k="143" />
<c g1="Y" g2="X" k="143" />
<c g1="Y" g2="X" k="96" />
<c g1="Y" g2="X" k="252" />
<c g1="Y" g2="X" k="143" />
<c g1="Y" g2="X" k="143" />
<c g1="Y" g2="X" k="96" />
<c g1="Y" g2="X" k="96" />
<c g1="Y" g2="X" k="96" />
<c g1="Z" g2="X" k="99" />
<c g1="Z" g2="X" k="99" />
<c g1="a" g2="X" k="92" />
<c g1="a" g2="X" k="45" />
<c g1="a" g2="X" k="50" />
<c g1="a" g2="X" k="45" />
<c g1="a" g2="X" k="72" />
<c g1="b" g2="X" k="43" />
<c g1="b" g2="X" k="49" />
<c g1="b" g2="X" k="43" />
<c g1="c" g2="X" k="9" />
<c g1="c" g2="X" k="9" />
<c g1="e" g2="X" k="47" />
<c g1="e" g2="X" k="29" />
<c g1="e" g2="X" k="35" />
<c g1="e" g2="X" k="-13" />
<c g1="e" g2="X" k="29" />
<c g1="e" g2="X" k="50" />
<c g1="f" g2="X" k="57" />
<c g1="f" g2="X" k="65" />
<c g1="f" g2="X" k="1" />
<c g1="f" g2="X" k="3" />
<c g1="f" g2="X" k="61" />
<c g1="f" g2="X" k="-50" />
<c g1="f" g2="X" k="66" />
<c g1="f" g2="X" k="20" />
<c g1="f" g2="X" k="1" />
<c g1="f" g2="X" k="57" />
<c g1="f" g2="X" k="31" />
<c g1="f" g2="X" k="57" />
<c g1="f" g2="X" k="65" />
<c g1="f" g2="X" k="66" />
<c g1="f" g2="X" k="31" />
<c g1="f" g2="X" k="57" />
<c g1="f" g2="X" k="34" />
<c g1="f" g2="X" k="65" />
<c g1="f" g2="X" k="10" />
<c g1="g" g2="X" k="43" />
<c g1="g" g2="X" k="62" />
<c g1="g" g2="X" k="0" />
<c g1="g" g2="X" k="-15" />
<c g1="g" g2="X" k="43" />
<c g1="g" g2="X" k="43" />
<c g1="g" g2="X" k="62" />
<c g1="g" g2="X" k="63" />
<c g1="g" g2="X" k="63" />
<c g1="g" g2="X" k="43" />
<c g1="h" g2="X" k="36" />
<c g1="h" g2="X" k="66" />
<c g1="i" g2="X" k="25" />
<c g1="i" g2="X" k="88" />
<c g1="k" g2="X" k="-3" />
<c g1="k" g2="X" k="114" />
<c g1="k" g2="X" k="-3" />
<c g1="k" g2="X" k="23" />
<c g1="k" g2="X" k="62" />
<c g1="k" g2="X" k="31" />
<c g1="k" g2="X" k="64" />
<c g1="k" g2="X" k="-14" />
<c g1="k" g2="X" k="4" />
<c g1="k" g2="X" k="23" />
<c g1="k" g2="X" k="23" />
<c g1="k" g2="X" k="23" />
<c g1="k" g2="X" k="62" />
<c g1="k" g2="X" k="64" />
<c g1="k" g2="X" k="64" />
<c g1="k" g2="X" k="4" />
<c g1="k" g2="X" k="23" />
<c g1="l" g2="X" k="20" />
<c g1="l" g2="X" k="20" />
<c g1="m" g2="X" k="12" />
<c g1="m" g2="X" k="38" />
<c g1="m" g2="X" k="44" />
<c g1="m" g2="X" k="38" />
<c g1="n" g2="X" k="119" />
<c g1="n" g2="X" k="11" />
<c g1="n" g2="X" k="36" />
<c g1="n" g2="X" k="41" />
<c g1="n" g2="X" k="36" />
<c g1="n" g2="X" k="66" />
<c g1="o" g2="X" k="192" />
<c g1="o" g2="X" k="35" />
<c g1="o" g2="X" k="54" />
<c g1="o" g2="X" k="60" />
<c g1="o" g2="X" k="60" />
<c g1="o" g2="X" k="54" />
<c g1="o" g2="X" k="65" />
<c g1="p" g2="X" k="35" />
<c g1="p" g2="X" k="43" />
<c g1="q" g2="X" k="18" />
<c g1="q" g2="X" k="16" />
<c g1="r" g2="X" k="135" />
<c g1="r" g2="X" k="2" />
<c g1="r" g2="X" k="135" />
<c g1="r" g2="X" k="65" />
<c g1="r" g2="X" k="65" />
<c g1="r" g2="X" k="59" />
<c g1="r" g2="X" k="17" />
<c g1="r" g2="X" k="18" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="3" />
<c g1="r" g2="X" k="25" />
<c g1="r" g2="X" k="54" />
<c g1="r" g2="X" k="5" />
<c g1="r" g2="X" k="63" />
<c g1="r" g2="X" k="54" />
<c g1="r" g2="X" k="59" />
<c g1="r" g2="X" k="0" />
<c g1="r" g2="X" k="0" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="-14" />
<c g1="r" g2="X" k="18" />
<c g1="r" g2="X" k="0" />
<c g1="r" g2="X" k="6" />
<c g1="r" g2="X" k="3" />
<c g1="r" g2="X" k="-11" />
<c g1="r" g2="X" k="-14" />
<c g1="r" g2="X" k="-9" />
<c g1="r" g2="X" k="-5" />
<c g1="r" g2="X" k="-14" />
<c g1="r" g2="X" k="29" />
<c g1="r" g2="X" k="59" />
<c g1="r" g2="X" k="59" />
<c g1="r" g2="X" k="59" />
<c g1="r" g2="X" k="59" />
<c g1="r" g2="X" k="59" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="59" />
<c g1="r" g2="X" k="15" />
<c g1="r" g2="X" k="14" />
<c g1="r" g2="X" k="22" />
<c g1="s" g2="X" k="39" />
<c g1="s" g2="X" k="80" />
<c g1="t" g2="X" k="58" />
<c g1="t" g2="X" k="58" />
<c g1="t" g2="X" k="12" />
<c g1="t" g2="X" k="24" />
<c g1="t" g2="X" k="56" />
<c g1="t" g2="X" k="-6" />
<c g1="t" g2="X" k="60" />
<c g1="t" g2="X" k="24" />
<c g1="t" g2="X" k="24" />
<c g1="t" g2="X" k="24" />
<c g1="t" g2="X" k="56" />
<c g1="t" g2="X" k="60" />
<c g1="t" g2="X" k="60" />
<c g1="t" g2="X" k="24" />
<c g1="t" g2="X" k="64" />
<c g1="u" g2="X" k="64" />
<c g1="v" g2="X" k="122" />
<c g1="v" g2="X" k="31" />
<c g1="v" g2="X" k="122" />
<c g1="v" g2="X" k="40" />
<c g1="v" g2="X" k="40" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="60" />
<c g1="v" g2="X" k="51" />
<c g1="v" g2="X" k="20" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="53" />
<c g1="v" g2="X" k="7" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="51" />
<c g1="v" g2="X" k="51" />
<c g1="v" g2="X" k="51" />
<c g1="v" g2="X" k="53" />
<c g1="v" g2="X" k="53" />
<c g1="v" g2="X" k="53" />
<c g1="v" g2="X" k="33" />
<c g1="v" g2="X" k="53" />
<c g1="w" g2="X" k="109" />
<c g1="w" g2="X" k="39" />
<c g1="w" g2="X" k="109" />
<c g1="w" g2="X" k="47" />
<c g1="w" g2="X" k="47" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="67" />
<c g1="w" g2="X" k="58" />
<c g1="w" g2="X" k="28" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="60" />
<c g1="w" g2="X" k="14" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="58" />
<c g1="w" g2="X" k="58" />
<c g1="w" g2="X" k="58" />
<c g1="w" g2="X" k="60" />
<c g1="w" g2="X" k="60" />
<c g1="w" g2="X" k="60" />
<c g1="w" g2="X" k="40" />
<c g1="w" g2="X" k="60" />
<c g1="x" g2="X" k="18" />
<c g1="x" g2="X" k="68" />
<c g1="x" g2="X" k="57" />
<c g1="x" g2="X" k="59" />
<c g1="x" g2="X" k="53" />
<c g1="x" g2="X" k="57" />
<c g1="y" g2="X" k="122" />
<c g1="y" g2="X" k="31" />
<c g1="y" g2="X" k="122" />
<c g1="y" g2="X" k="40" />
<c g1="y" g2="X" k="40" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="60" />
<c g1="y" g2="X" k="51" />
<c g1="y" g2="X" k="20" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="53" />
<c g1="y" g2="X" k="7" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="51" />
<c g1="y" g2="X" k="51" />
<c g1="y" g2="X" k="51" />
<c g1="y" g2="X" k="53" />
<c g1="y" g2="X" k="53" />
<c g1="y" g2="X" k="53" />
<c g1="y" g2="X" k="33" />
<c g1="y" g2="X" k="53" />
<c g1="Adieresis" g2="X" k="-42" />
<c g1="Adieresis" g2="X" k="30" />
<c g1="Adieresis" g2="X" k="-42" />
<c g1="Adieresis" g2="X" k="101" />
<c g1="Adieresis" g2="X" k="101" />
<c g1="Adieresis" g2="X" k="97" />
<c g1="Adieresis" g2="X" k="97" />
<c g1="Adieresis" g2="X" k="101" />
<c g1="Adieresis" g2="X" k="101" />
<c g1="Adieresis" g2="X" k="214" />
<c g1="Adieresis" g2="X" k="190" />
<c g1="Adieresis" g2="X" k="138" />
<c g1="Adieresis" g2="X" k="1" />
<c g1="Adieresis" g2="X" k="-1" />
<c g1="Adieresis" g2="X" k="50" />
<c g1="Adieresis" g2="X" k="35" />
<c g1="Adieresis" g2="X" k="11" />
<c g1="Adieresis" g2="X" k="41" />
<c g1="Adieresis" g2="X" k="35" />
<c g1="Adieresis" g2="X" k="47" />
<c g1="Adieresis" g2="X" k="28" />
<c g1="Adieresis" g2="X" k="143" />
<c g1="Adieresis" g2="X" k="117" />
<c g1="Adieresis" g2="X" k="143" />
<c g1="Adieresis" g2="X" k="72" />
<c g1="Adieresis" g2="X" k="198" />
<c g1="Adieresis" g2="X" k="221" />
<c g1="Adieresis" g2="X" k="101" />
<c g1="Aring" g2="X" k="-42" />
<c g1="Aring" g2="X" k="30" />
<c g1="Aring" g2="X" k="-42" />
<c g1="Aring" g2="X" k="101" />
<c g1="Aring" g2="X" k="101" />
<c g1="Aring" g2="X" k="97" />
<c g1="Aring" g2="X" k="97" />
<c g1="Aring" g2="X" k="101" />
<c g1="Aring" g2="X" k="101" />
<c g1="Aring" g2="X" k="214" />
<c g1="Aring" g2="X" k="190" />
<c g1="Aring" g2="X" k="138" />
<c g1="Aring" g2="X" k="1" />
<c g1="Aring" g2="X" k="-1" />
<c g1="Aring" g2="X" k="50" />
<c g1="Aring" g2="X" k="35" />
<c g1="Aring" g2="X" k="40" />
<c g1="Aring" g2="X" k="11" />
<c g1="Aring" g2="X" k="41" />
<c g1="Aring" g2="X" k="35" />
<c g1="Aring" g2="X" k="47" />
<c g1="Aring" g2="X" k="28" />
<c g1="Aring" g2="X" k="143" />
<c g1="Aring" g2="X" k="117" />
<c g1="Aring" g2="X" k="143" />
<c g1="Aring" g2="X" k="72" />
<c g1="Aring" g2="X" k="198" />
<c g1="Aring" g2="X" k="221" />
<c g1="Aring" g2="X" k="101" />
<c g1="Ccedilla" g2="X" k="-27" />
<c g1="Odieresis" g2="X" k="101" />
<c g1="Odieresis" g2="X" k="30" />
<c g1="Odieresis" g2="X" k="95" />
<c g1="Odieresis" g2="X" k="98" />
<c g1="Odieresis" g2="X" k="101" />
<c g1="Odieresis" g2="X" k="96" />
<c g1="Udieresis" g2="X" k="33" />
<c g1="Udieresis" g2="X" k="27" />
<c g1="Udieresis" g2="X" k="92" />
<c g1="Udieresis" g2="X" k="-36" />
<c g1="Udieresis" g2="X" k="44" />
<c g1="Udieresis" g2="X" k="44" />
<c g1="Udieresis" g2="X" k="32" />
<c g1="Udieresis" g2="X" k="44" />
<c g1="aacute" g2="X" k="45" />
<c g1="aacute" g2="X" k="50" />
<c g1="aacute" g2="X" k="45" />
<c g1="agrave" g2="X" k="45" />
<c g1="agrave" g2="X" k="50" />
<c g1="agrave" g2="X" k="45" />
<c g1="adieresis" g2="X" k="45" />
<c g1="adieresis" g2="X" k="50" />
<c g1="adieresis" g2="X" k="45" />
<c g1="aring" g2="X" k="45" />
<c g1="aring" g2="X" k="50" />
<c g1="aring" g2="X" k="45" />
<c g1="eacute" g2="X" k="29" />
<c g1="eacute" g2="X" k="35" />
<c g1="eacute" g2="X" k="29" />
<c g1="ecircumflex" g2="X" k="29" />
<c g1="ecircumflex" g2="X" k="35" />
<c g1="ecircumflex" g2="X" k="29" />
<c g1="oacute" g2="X" k="54" />
<c g1="oacute" g2="X" k="60" />
<c g1="oacute" g2="X" k="54" />
<c g1="ograve" g2="X" k="54" />
<c g1="ograve" g2="X" k="60" />
<c g1="ograve" g2="X" k="54" />
<c g1="ocircumflex" g2="X" k="35" />
<c g1="odieresis" g2="X" k="35" />
<c g1="odieresis" g2="X" k="54" />
<c g1="odieresis" g2="X" k="60" />
<c g1="odieresis" g2="X" k="60" />
<c g1="odieresis" g2="X" k="54" />
<c g1="Oslash" g2="X" k="101" />
<c g1="ae" g2="X" k="33" />
<c g1="ae" g2="X" k="39" />
<c g1="ae" g2="X" k="33" />
<c g1="guillemotright" g2="X" k="77" />
<c g1="guillemotright" g2="X" k="200" />
<c g1="guillemotright" g2="X" k="161" />
<c g1="guillemotright" g2="X" k="127" />
<c g1="guillemotright" g2="X" k="218" />
<c g1="guillemotright" g2="X" k="77" />
<c g1="guillemotright" g2="X" k="77" />
<c g1="guillemotright" g2="X" k="106" />
<c g1="guillemotright" g2="X" k="77" />
<c g1="Agrave" g2="X" k="-42" />
<c g1="Agrave" g2="X" k="-42" />
<c g1="Agrave" g2="X" k="101" />
<c g1="Agrave" g2="X" k="101" />
<c g1="Agrave" g2="X" k="97" />
<c g1="Agrave" g2="X" k="97" />
<c g1="Agrave" g2="X" k="101" />
<c g1="Agrave" g2="X" k="101" />
<c g1="Agrave" g2="X" k="214" />
<c g1="Agrave" g2="X" k="190" />
<c g1="Agrave" g2="X" k="138" />
<c g1="Atilde" g2="X" k="-42" />
<c g1="Atilde" g2="X" k="-42" />
<c g1="Atilde" g2="X" k="101" />
<c g1="Atilde" g2="X" k="101" />
<c g1="Atilde" g2="X" k="97" />
<c g1="Atilde" g2="X" k="97" />
<c g1="Atilde" g2="X" k="101" />
<c g1="Atilde" g2="X" k="101" />
<c g1="Atilde" g2="X" k="214" />
<c g1="Atilde" g2="X" k="190" />
<c g1="Atilde" g2="X" k="138" />
<c g1="Otilde" g2="X" k="30" />
<c g1="Otilde" g2="X" k="95" />
<c g1="Otilde" g2="X" k="96" />
<c g1="quotedblleft" g2="X" k="202" />
<c g1="quotedblleft" g2="X" k="19" />
<c g1="quotedblleft" g2="X" k="6" />
<c g1="quotedblleft" g2="X" k="10" />
<c g1="quotedblleft" g2="X" k="10" />
<c g1="quotedblleft" g2="X" k="202" />
<c g1="quotedblleft" g2="X" k="202" />
<c g1="quotedblleft" g2="X" k="293" />
<c g1="quotedblleft" g2="X" k="202" />
<c g1="quotedblright" g2="X" k="202" />
<c g1="quotedblright" g2="X" k="6" />
<c g1="quotedblright" g2="X" k="-13" />
<c g1="quotedblright" g2="X" k="-6" />
<c g1="quotedblright" g2="X" k="-12" />
<c g1="quotedblright" g2="X" k="202" />
<c g1="quotedblright" g2="X" k="202" />
<c g1="quotedblright" g2="X" k="295" />
<c g1="quotedblright" g2="X" k="202" />
<c g1="quoteleft" g2="X" k="223" />
<c g1="quoteleft" g2="X" k="42" />
<c g1="quoteleft" g2="X" k="32" />
<c g1="quoteleft" g2="X" k="35" />
<c g1="quoteleft" g2="X" k="33" />
<c g1="quoteleft" g2="X" k="223" />
<c g1="quoteleft" g2="X" k="223" />
<c g1="quoteleft" g2="X" k="311" />
<c g1="quoteleft" g2="X" k="223" />
<c g1="quoteright" g2="X" k="101" />
<c g1="quoteright" g2="X" k="101" />
<c g1="quoteright" g2="X" k="223" />
<c g1="quoteright" g2="X" k="87" />
<c g1="quoteright" g2="X" k="81" />
<c g1="quoteright" g2="X" k="60" />
<c g1="quoteright" g2="X" k="50" />
<c g1="quoteright" g2="X" k="63" />
<c g1="quoteright" g2="X" k="47" />
<c g1="quoteright" g2="X" k="53" />
<c g1="quoteright" g2="X" k="47" />
<c g1="quoteright" g2="X" k="223" />
<c g1="quoteright" g2="X" k="223" />
<c g1="quoteright" g2="X" k="315" />
<c g1="quoteright" g2="X" k="223" />
<c g1="guilsinglright" g2="X" k="104" />
<c g1="guilsinglright" g2="X" k="228" />
<c g1="guilsinglright" g2="X" k="188" />
<c g1="guilsinglright" g2="X" k="155" />
<c g1="guilsinglright" g2="X" k="246" />
<c g1="guilsinglright" g2="X" k="104" />
<c g1="guilsinglright" g2="X" k="104" />
<c g1="guilsinglright" g2="X" k="134" />
<c g1="guilsinglright" g2="X" k="104" />
<c g1="quotedblbase" g2="X" k="-27" />
<c g1="quotedblbase" g2="X" k="155" />
<c g1="quotedblbase" g2="X" k="207" />
<c g1="quotedblbase" g2="X" k="153" />
<c g1="quotedblbase" g2="X" k="183" />
<c g1="quotedblbase" g2="X" k="-21" />
<c g1="Acircumflex" g2="X" k="-42" />
<c g1="Acircumflex" g2="X" k="-42" />
<c g1="Acircumflex" g2="X" k="101" />
<c g1="Acircumflex" g2="X" k="101" />
<c g1="Acircumflex" g2="X" k="97" />
<c g1="Acircumflex" g2="X" k="97" />
<c g1="Acircumflex" g2="X" k="101" />
<c g1="Acircumflex" g2="X" k="101" />
<c g1="Acircumflex" g2="X" k="214" />
<c g1="Acircumflex" g2="X" k="190" />
<c g1="Acircumflex" g2="X" k="138" />
<c g1="Aacute" g2="X" k="-42" />
<c g1="Aacute" g2="X" k="30" />
<c g1="Aacute" g2="X" k="-42" />
<c g1="Aacute" g2="X" k="101" />
<c g1="Aacute" g2="X" k="101" />
<c g1="Aacute" g2="X" k="97" />
<c g1="Aacute" g2="X" k="97" />
<c g1="Aacute" g2="X" k="101" />
<c g1="Aacute" g2="X" k="101" />
<c g1="Aacute" g2="X" k="214" />
<c g1="Aacute" g2="X" k="190" />
<c g1="Aacute" g2="X" k="138" />
<c g1="Aacute" g2="X" k="1" />
<c g1="Aacute" g2="X" k="-1" />
<c g1="Aacute" g2="X" k="50" />
<c g1="Aacute" g2="X" k="35" />
<c g1="Aacute" g2="X" k="40" />
<c g1="Aacute" g2="X" k="11" />
<c g1="Aacute" g2="X" k="41" />
<c g1="Aacute" g2="X" k="35" />
<c g1="Aacute" g2="X" k="47" />
<c g1="Aacute" g2="X" k="28" />
<c g1="Aacute" g2="X" k="143" />
<c g1="Aacute" g2="X" k="117" />
<c g1="Aacute" g2="X" k="143" />
<c g1="Aacute" g2="X" k="72" />
<c g1="Aacute" g2="X" k="221" />
<c g1="Aacute" g2="X" k="101" />
<c g1="Oacute" g2="X" k="101" />
<c g1="Oacute" g2="X" k="30" />
<c g1="Oacute" g2="X" k="95" />
<c g1="Oacute" g2="X" k="98" />
<c g1="Oacute" g2="X" k="96" />
<c g1="Ocircumflex" g2="X" k="30" />
<c g1="Ocircumflex" g2="X" k="95" />
<c g1="Ocircumflex" g2="X" k="96" />
<c g1="Ograve" g2="X" k="30" />
<c g1="Ograve" g2="X" k="95" />
<c g1="Ograve" g2="X" k="96" />
<c g1="Uacute" g2="X" k="33" />
<c g1="Uacute" g2="X" k="27" />
<c g1="Uacute" g2="X" k="92" />
<c g1="Uacute" g2="X" k="44" />
<c g1="Uacute" g2="X" k="44" />
<c g1="Uacute" g2="X" k="32" />
<c g1="Uacute" g2="X" k="44" />
<c g1="Ucircumflex" g2="X" k="92" />
<c g1="Ugrave" g2="X" k="92" />
</font>
</defs>
</svg>
"##).unwrap();//"//
    let mut xpath = libxml::xpath::Context::new(&doc).unwrap();
    xpath.register_namespace("svg", "http://www.w3.org/2000/svg").unwrap();
    for mut k in xpath.findnodes("//svg:c", None).unwrap() {
        match (glyph_string, k.get_attribute("g1"), k.get_attribute("g2")) {
            ("10", Some(ref g1),Some(ref g2)) if g1 == "one" && g2 == "zero" => {},
            _ => k.unlink_node(),
        }
    }
    let fonts = xpath.findnodes("//svg:font", None).unwrap();
    let font = fonts.first().unwrap();
    doc.node_to_string(font)
}

#[test]
fn flex_drop() {
    for _ in 0..1000 {
        flex_drop_fn_2("8");
    }
}

#[test]
/// Can mock a node object (useful for defaults that will be overridden)
fn can_mock_node() {
  let doc_mock = Document::new().unwrap();
  let node_mock = Node::mock(&doc_mock);
  assert!(!node_mock.is_text_node());
}

#[test]
/// Can make a mock node hashable
fn can_hash_mock_node() {
  let doc_mock = Document::new().unwrap();
  let node_mock = Node::mock(&doc_mock);
  assert!(node_mock.to_hashable() > 0);
}

#[test]
/// Can make null nodes and documents, to avoid memory allocations
fn can_null_node() {
  let null_node = Node::null();
  let second_null_node = Node::null();
  assert!(null_node.is_null());
  assert!(second_null_node.is_null());
  assert_eq!(null_node, second_null_node);
}

#[test]
/// Can set and get attributes
fn can_manage_attributes() {
  let mut doc = Document::new().unwrap();
  let hello_element_result = Node::new("hello", None, &doc);
  assert!(hello_element_result.is_ok());
  let mut hello_element = hello_element_result.unwrap();
  doc.set_root_element(&hello_element);

  let key = "examplekey";
  let value = "examplevalue";
  let pre_value = hello_element.get_attribute(key);
  assert_eq!(pre_value, None);
  let pre_prop_value = hello_element.get_property(key);
  assert_eq!(pre_prop_value, None);

  assert!(hello_element.set_attribute(key, value).is_ok());
  let new_value = hello_element.get_attribute(key);
  assert_eq!(new_value, Some(value.to_owned()));
}

#[test]
/// Can set and get text node content
fn can_set_get_text_node_content() {
  let mut doc = Document::new().unwrap();
  let hello_element_result = Node::new("hello", None, &doc);
  assert!(hello_element_result.is_ok());
  let mut hello_element = hello_element_result.unwrap();
  doc.set_root_element(&hello_element);

  assert!(hello_element.get_content().is_empty());
  assert!(hello_element.append_text("hello ").is_ok());
  assert_eq!(hello_element.get_content(), "hello ");
  assert!(hello_element.append_text("world!").is_ok());
  assert_eq!(hello_element.get_content(), "hello world!");
}

#[test]
/// Basic namespace workflow
fn can_work_with_namespaces() {
  let mut doc = Document::new().unwrap();
  let mut root_node = Node::new("root", None, &doc).unwrap();
  doc.set_root_element(&root_node);

  let initial_namespace_list = root_node.get_namespaces(&doc);
  assert_eq!(initial_namespace_list.len(), 0);

  let mock_ns_result = Namespace::new("mock", "http://example.com/ns/mock", &mut root_node);
  assert!(mock_ns_result.is_ok());
  let second_ns_result = Namespace::new("second", "http://example.com/ns/second", &mut root_node);
  assert!(second_ns_result.is_ok());

  // try to attach this namespace to a node
  assert!(root_node.get_namespace().is_none());
  assert!(root_node.set_namespace(&mock_ns_result.unwrap()).is_ok());
  let active_ns_opt = root_node.get_namespace();
  assert!(active_ns_opt.is_some());
  let active_ns = active_ns_opt.unwrap();
  assert_eq!(active_ns.get_prefix(), "mock");
  assert_eq!(active_ns.get_href(), "http://example.com/ns/mock");

  // now get all namespaces for the node and check we have ours
  let mut namespace_list = root_node.get_namespaces(&doc);
  assert_eq!(namespace_list.len(), 2);

  let second_ns = namespace_list.pop().unwrap();
  assert_eq!(second_ns.get_prefix(), "second");
  assert_eq!(second_ns.get_href(), "http://example.com/ns/second");

  let first_ns = namespace_list.pop().unwrap();
  assert_eq!(first_ns.get_prefix(), "mock");
  assert_eq!(first_ns.get_href(), "http://example.com/ns/mock");
}

#[test]
fn can_work_with_ns_declarations() {
  let mut doc = Document::new().unwrap();
  let mut root_node = Node::new("root", None, &doc).unwrap();
  doc.set_root_element(&root_node);

  let mock_ns_result = Namespace::new("mock1", "http://example.com/ns/mock1", &mut root_node);
  assert!(mock_ns_result.is_ok());
  let second_ns_result = Namespace::new("mock2", "http://example.com/ns/mock2", &mut root_node);
  assert!(second_ns_result.is_ok());

  let declarations = root_node.get_namespace_declarations();
  assert_eq!(declarations.len(), 2);
}

#[test]
/// Can view documents as nodes
fn can_cast_doc_to_node() {
  // Setup
  let parser = Parser::default();
  let doc_result = parser.parse_file("tests/resources/file01.xml");
  assert!(doc_result.is_ok());

  let doc = doc_result.unwrap();
  let doc_node = doc.as_node();
  assert_eq!(doc_node.get_type(), Some(NodeType::DocumentNode));
  let root_node_opt = doc_node.get_first_child();
  assert!(root_node_opt.is_some());
  let root_node = root_node_opt.unwrap();
  assert_eq!(root_node.get_name(), "root");
}

#[test]
fn can_replace_child() {
  let mut doc = Document::new().unwrap();
  let mut root_node = Node::new("root", None, &doc).unwrap();
  doc.set_root_element(&root_node);
  let mut a = Node::new("a", None, &doc).unwrap();
  let mut b = Node::new("b", None, &doc).unwrap();
  let mut c = Node::new("c", None, &doc).unwrap();
  let mut d = Node::new("d", None, &doc).unwrap();
  let mut e = Node::new("e", None, &doc).unwrap();

  assert!(root_node.add_child(&mut a).is_ok());
  assert!(root_node.add_child(&mut b).is_ok());
  assert!(root_node.add_child(&mut c).is_ok());
  assert!(root_node.add_child(&mut d).is_ok());
  assert!(root_node.add_child(&mut e).is_ok());
  assert_eq!(doc.to_string(false),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><a/><b/><c/><d/><e/></root>\n",
    "document initialized correctly.");

  // replace first child with new F
  let f = Node::new("F", None, &doc).unwrap();
  let a_result = root_node.replace_child_node(f, a);
  assert!(a_result.is_ok());
  
  assert_eq!(doc.to_string(false),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><c/><d/><e/></root>\n",
    "document initialized correctly.");

  // replace last child with new G
  let g = Node::new("G", None, &doc).unwrap();
  assert!(root_node.replace_child_node(g, e).is_ok());
  assert_eq!(doc.to_string(false),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><c/><d/><G/></root>\n",
    "document initialized correctly.");

  // replace middle child with new H
  let h = Node::new("H", None, &doc).unwrap();
  assert!(root_node.replace_child_node(h, c).is_ok());
  assert_eq!(doc.to_string(false),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><H/><d/><G/></root>\n",
    "document initialized correctly.");

  // fail to replace a, as it is already removed.
  let none = Node::new("none", None, &doc).unwrap();
  assert!(root_node.replace_child_node(none, a_result.unwrap()).is_err());
  // no change.
  assert_eq!(doc.to_string(false),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><H/><d/><G/></root>\n",
    "document initialized correctly.");

  // replacing with self succeeds without change.
  assert!(root_node.replace_child_node(b.clone(),b).is_ok());
  assert_eq!(doc.to_string(false),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><H/><d/><G/></root>\n",
    "document initialized correctly.");
  // replacing with parent succeeds without change.
  assert!(root_node.replace_child_node(root_node.clone(),d).is_ok());
  assert_eq!(doc.to_string(false),
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root><F/><b/><H/><d/><G/></root>\n",
    "document initialized correctly.");
}