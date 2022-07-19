
#[derive(Debug, PartialEq)]
enum FundamentalType {
    boolean,
    byte,
    integer,
    float,
    string,
    defined_type
}

#[derive(Debug, PartialEq)]
struct Type {
    ty: FundamentalType;
}

trait NodeType {
    fn check(&self) -> Result<Type>;
}

impl NodeType for expression::boolean {
    fn check(&self) -> Result<Type> {
        Ok(Type{ ty: FundamentalType::boolean })
    }
}

impl NodeType for expression::byte {
    fn check(&self) -> Result<Type> {
        Ok(Type{ ty: FundamentalType::byte })
    }
}

impl NodeType for expression::float {
    fn check(&self) -> Result<Type> {
        Ok(Type{ ty: FundamentalType::float })
    }
}

impl NodeType for expression::integer {
    fn check(&self) -> Result<Type> {
        Ok(Type{ ty: FundamentalType::integer })
    }
}

impl NodeType for expression::string {
    fn check(&self) -> Result<Type> {
        Ok(Type{ ty: FundamentalType::string })
    }
}

impl NodeType for expression::binary_operator {
    fn check(&self) -> Result<Type> {
        let left_type = self.left.check();
        let right_type = self.right.check();

        // match self.op.value {
        //     "+" => {
        //         if left_type.ty == 
        //     }
        // }
    }
}