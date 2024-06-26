mod add_classname;

use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    ecma::{
        ast::Program,
        visit::{as_folder, FoldWith},
    },
    plugin::metadata::TransformPluginMetadataContextKind,
};

use add_classname::AddClassnameVisitor;

#[plugin_transform]
pub fn process_transform(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let filepath = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s,
        None => String::from(""),
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
        /* Output */
        r#"
        const MyComponent = () => <Component className="file-name-component" />;
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ use_state_no_classname_yet,
        /* Input */
        r#"
        export const LoginTextField = (props: TextFieldProps) => {
            const [filename, setFilename] = useState("file.txt");

            return <TextField id={{filename}} />;
          };
        "#,
        /* Output */
        r#"
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
        /* Input */
        r#"
          export const LoginTextField = (props: TextFieldProps) => {
            const [filename, setFilename] = useState("file.txt");

            return <TextField className="no-print" id={{filename}} />;
          };
        "#,
        /* Output */
        r#"
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
        /* Input */
        r#"
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
        /* Output */
        r#"
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
        /* Input */
        r#"
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
        /* Output */
        r#"
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

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ complex_class_assignment,
        /* Input */
        r#"
        export const GridComponent = (props: TextFieldProps) => (
          <>
            <GridApiRefContext.Provider value={gridApiRef}>
              {props.children}
            </GridApiRefContext.Provider>
            <div
              className={`
                ag-theme-alpine
                ${
                  gridProps.onRowClicked || gridProps.onRowSelected
                    ? "clickable-rows"
                    : ""
                }
                ${className || ""}
              `}
              style={{
                height: height ? height : staticHeight ? staticHeight : "100%",
                width: "100%",
                overflow: "visible",
              }}
            >
              <AgGridReact
                defaultColDef={{
                  sortable: true,
                  wrapHeaderText: true,
                  ...props.defaultColDef,
                }}
                enableCellTextSelection
                suppressCellFocus
                suppressMovableColumns={suppressMovableColumns}
                onGridReady={gridReady}
                onColumnResized={handleColumnResized}
                isExternalFilterPresent={() => !!ids.current && enableFilters}
                doesExternalFilterPass={node =>
                  ids.current ? ids.current.includes(node.data.id) : true
                }
                {...gridProps}
              />
            </div>
          </>
        );
      "#,
        /* Output */
        r#"
        export const GridComponent = (props: TextFieldProps) =>
          <>
            <GridApiRefContext.Provider value={gridApiRef} className="file-name-provider">
              {props.children}
            </GridApiRefContext.Provider>
            <div
              className={`
                ag-theme-alpine
                ${
                  gridProps.onRowClicked || gridProps.onRowSelected
                    ? "clickable-rows"
                    : ""
                }
                ${className || ""}
              `}
              style={{
                height: height ? height : staticHeight ? staticHeight : "100%",
                width: "100%",
                overflow: "visible",
              }}
            >
              <AgGridReact
                defaultColDef={{
                  sortable: true,
                  wrapHeaderText: true,
                  ...props.defaultColDef,
                }}
                enableCellTextSelection
                suppressCellFocus
                suppressMovableColumns={suppressMovableColumns}
                onGridReady={gridReady}
                onColumnResized={handleColumnResized}
                isExternalFilterPresent={() => !!ids.current && enableFilters}
                doesExternalFilterPass={node =>
                  ids.current ? ids.current.includes(node.data.id) : true
                }
                {...gridProps}
                className="file-name-ag-grid-react"
              />
            </div>
          </>;
      "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ fragment_no_classname,
        /* Input */
        r#"
        export const GridComponent = (props: TextFieldProps) => (
          <>
            Some text for this fragment
          </>
        );
      "#,
        /* Output */
        r#"
        export const GridComponent = (props: TextFieldProps) =>
          <>
            Some text for this fragment
          </>;
      "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ fragment_literal_no_classname,
        /* Input */
        r#"
          export const GridComponent = (props: TextFieldProps) => (
            <Fragment>
              Some text for this fragment
            </Fragment>
          );
        "#,
        /* Output */
        r#"
          export const GridComponent = (props: TextFieldProps) =>
            <Fragment>
              Some text for this fragment
            </Fragment>;
          "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ component_within_props,
        /* Input */
        r#"
          <Row alignRight gap="50px" style={{ marginTop: "32px" }}>
            <ModalOpener
              Opener={({ onOpen }) => (
                <PrimaryButton
                  scale="big"
                  text={display.buttonText}
                  onClick={(e: React.MouseEvent<HTMLButtonElement>) => {
                    e.stopPropagation();
                    onOpen();
                  }}
                />
              )}
              Modal={({ onClose }) => (
                <ConfirmationDialog
                  onCancel={onClose}
                  text={display.modalText}
                  onConfirm={async (event: any) => {
                    event.preventDefault();
                    await updateVendor();
                  }}
                />
              )}
            />
          </Row>
        "#,
        /* Output */
        r#"
          <Row alignRight gap="50px" style={{ marginTop: "32px" }} className="file-name-row">
            <ModalOpener
              Opener={({ onOpen }) =>
                <PrimaryButton
                  scale="big"
                  text={display.buttonText}
                  onClick={(e: React.MouseEvent<HTMLButtonElement>) => {
                    e.stopPropagation();
                    onOpen();
                  }}
                  className="file-name-primary-button"
                />
              }
              Modal={({ onClose }) =>
                <ConfirmationDialog
                  onCancel={onClose}
                  text={display.modalText}
                  onConfirm={async (event: any) => {
                    event.preventDefault();
                    await updateVendor();
                  }}
                  className="file-name-confirmation-dialog"
                />
              }
              className="file-name-modal-opener"
            />
          </Row>
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ component_within_component_within_props,
        /* Input */
        r#"
          <Row alignRight gap="50px" style={{ marginTop: "32px" }}>
            <ModalOpener
              Opener={({ onOpen }) => (
                <PrimaryButton
                  scale="big"
                  text={display.buttonText}
                  onClick={(e: React.MouseEvent<HTMLButtonElement>) => {
                    e.stopPropagation();
                    onOpen();
                  }}
                />
              )}
              Modal={({ onClose }) => (
                <ConfirmationDialog
                  onCancel={onClose}
                  text={display.modalText}
                  onConfirm={async (event: any) => {
                    event.preventDefault();
                    await updateVendor();
                  }}
                >
                  <SubDialog className="no-print" />
                </ConfirmationDialog>
              )}
            />
          </Row>
        "#,
        /* Output */
        r#"
          <Row alignRight gap="50px" style={{ marginTop: "32px" }} className="file-name-row">
            <ModalOpener
              Opener={({ onOpen }) =>
                <PrimaryButton
                  scale="big"
                  text={display.buttonText}
                  onClick={(e: React.MouseEvent<HTMLButtonElement>) => {
                    e.stopPropagation();
                    onOpen();
                  }}
                  className="file-name-primary-button"
                />
              }
              Modal={({ onClose }) =>
                <ConfirmationDialog
                  onCancel={onClose}
                  text={display.modalText}
                  onConfirm={async (event: any) => {
                    event.preventDefault();
                    await updateVendor();
                  }}
                  className="file-name-confirmation-dialog"
                >
                  <SubDialog className="no-print file-name-sub-dialog" />
                </ConfirmationDialog>
              }
              className="file-name-modal-opener"
            />
          </Row>
        "#
    );

    test_inline!(
        SYNTAX,
        runner,
        /* Name */ class_component_with_render_route,
        /* Input */
        r#"
          class NotificationsAlert extends React.Component<Props> {
            public constructor(props: Props) {
              super(props);
            }

            public render() {
              return (
                <Link to="/notifications">
                  <Route
                    path="/notifications"
                    children={route => (
                      <AlertWrapper>
                        <Icon icon={faBell} color={colors.white} />
                        {!!this.props.hasUnreadNotifications && !route.match && (
                          <AlertBubble />
                        )}
                      </AlertWrapper>
                    )}
                  />
                </Link>
              );
            }
          }
        "#,
        /* Output */
        r#"
          class NotificationsAlert extends React.Component<Props> {
            public constructor(props: Props) {
              super(props);
            }

            public render() {
              return <Link to="/notifications" className="file-name-link">
                  <Route
                    path="/notifications"
                    children={route =>
                      <AlertWrapper className="file-name-alert-wrapper">
                        <Icon icon={faBell} color={colors.white} className="file-name-icon"/>
                        {!!this.props.hasUnreadNotifications && !route.match &&
                          <AlertBubble className="file-name-alert-bubble"/>
                        }
                      </AlertWrapper>
                    }
                    className="file-name-route"
                  />
                </Link>;
            }
          }
        "#
    );
}
