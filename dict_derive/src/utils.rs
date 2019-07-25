use syn::GenericArgument;
use syn::PathArguments;

#[derive(PartialEq, Eq, Debug)]
pub enum MappedFieldType {
    IsBox,
    IsOption,
    IsOptionBox,
    IsAny,
}

pub fn which_field_type(ty: &syn::Type) -> MappedFieldType {
    let path = match *ty {
        syn::Type::Path(ref p) if p.qself.is_none() => &p.path,
        _ => return MappedFieldType::IsAny,
    };

    let mut segments_iter = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string());

    if segments_iter.clone().any(|x| x.as_str() == "Box") {
        MappedFieldType::IsBox
    } else if segments_iter.any(|x| x.as_str() == "Option") {
        match path.segments[0].arguments {
            PathArguments::AngleBracketed(ref angle_bracketed) => {
                if angle_bracketed.args.len() != 1 {
                    panic!("Didn't expected more than one AngleBracketed");
                }

                let argument = &angle_bracketed.args[0];

                let field_type = match argument {
                    GenericArgument::Type(ref t) => which_field_type(&t),
                    _ => panic!(format!("Unexpected GenericArgument {:?}", argument)),
                };

                match field_type {
                    MappedFieldType::IsBox => MappedFieldType::IsOptionBox,
                    _ => MappedFieldType::IsOption,
                }
            }
            PathArguments::None => MappedFieldType::IsAny,
            _ => panic!(format!(
                "Unexpected segment {:?}",
                path.segments[0].arguments
            )),
        }
    } else {
        MappedFieldType::IsAny
    }
}
