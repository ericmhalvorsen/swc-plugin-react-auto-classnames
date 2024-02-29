mod add_classname;

use swc_core::{ecma::{
    ast::Program,
    visit::{as_folder, FoldWith},
}, plugin::metadata::TransformPluginMetadataContextKind};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

use add_classname::AddClassnameVisitor;

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let filepath = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s,
        None => String::from("")
    };
    program.fold_with(&mut as_folder(AddClassnameVisitor::new(&filepath)))
}

#[cfg(test)]
mod test {
    use swc_core::common::{chain, Mark};
    use swc_core::ecma::transforms::base::resolver;
    use swc_core::ecma::transforms::testing::Tester;
    use swc_core::ecma::{
        parser::{Syntax, TsConfig},
        transforms::testing::test_inline,
        visit::{as_folder, Fold},
    };

    const SYNTAX: Syntax = Syntax::Typescript(TsConfig {
        tsx: true,
        decorators: false,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: true,
    });

    fn runner(_: &mut Tester) -> impl Fold {
        chain!(
            resolver(Mark::new(), Mark::new(), false),
            as_folder(super::AddClassnameVisitor::new("lib/File_Name.tsx"))
        )
    }

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ simple_example,
        /* Input */ r#"
        const MyComponent = () => <Component />;
        "#,
        /* Output */ r#"
        const MyComponent = () => <Component className="file-name-component" />;
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ use_state_no_classname_yet,
        /* Input */ r#"
        export const LoginTextField = (props: TextFieldProps) => {
            const [filename, setFilename] = useState("file.txt");

            return <TextField id={{filename}} />;
          };
        "#,
        /* Output */ r#"
          export const LoginTextField = (props: TextFieldProps) => {
            const [filename, setFilename] = useState("file.txt");

            return <TextField id={{filename}} className="file-name-text-field" />;
          };
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ use_state_with_classname,
        /* Input */ r#"
          export const LoginTextField = (props: TextFieldProps) => {
            const [filename, setFilename] = useState("file.txt");

            return <TextField className="no-print" id={{filename}} />;
          };
        "#,
        /* Output */ r#"
          export const LoginTextField = (props: TextFieldProps) => {
            const [filename, setFilename] = useState("file.txt");

            return <TextField className="no-print file-name-text-field" id={{filename}} />;
          };
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ complex_no_classname_yet,
        /* Input */ r#"
          export const LoginTextField = (props: TextFieldProps) => (
            <TextField
              variant="outlined"
              role="presentation"
              focused
              color="secondary"
              {...props}
              style={{ margin: "1em 0", width: "100%", ...(props.style || {}) }}
              InputProps={{
                style: { fontSize: "20px", ...(props.InputProps?.style || {}) },
                ...(props.InputProps || {}),
              }}
            />
          );
        "#,
        /* Output */ r#"
          export const LoginTextField = (props: TextFieldProps) =>
            <TextField
              variant="outlined"
              role="presentation"
              focused
              color="secondary"
              {...props}
              style={{ margin: "1em 0", width: "100%", ...props.style || {} }}
              InputProps={{
                style: { fontSize: "20px", ...props.InputProps?.style || {} },
                ...props.InputProps || {},
              }}
              className="file-name-text-field"
            />;
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ complex_with_classname,
        /* Input */ r#"
          export const LoginTextField = (props: TextFieldProps) => (
            <TextField
              variant="outlined"
              className="no-print"
              role="presentation"
              focused
              color="secondary"
              {...props}
              style={{ margin: "1em 0", width: "100%", ...(props.style || {}) }}
              InputProps={{
                style: { fontSize: "20px", ...(props.InputProps?.style || {}) },
                ...(props.InputProps || {}),
              }}

            />
          );
        "#,
        /* Output */ r#"
          export const LoginTextField = (props: TextFieldProps) =>
            <TextField
              variant="outlined"
              className="no-print file-name-text-field"
              role="presentation"
              focused
              color="secondary"
              {...props}
              style={{ margin: "1em 0", width: "100%", ...props.style || {} }}
              InputProps={{
                style: { fontSize: "20px", ...props.InputProps?.style || {} },
                ...props.InputProps || {},
              }}
            />;
        "#
    );
}
