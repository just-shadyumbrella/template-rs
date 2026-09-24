use std::fs;

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let app_icon = "app.ico";
    // let app_icon = "";
    let manifest = "app.manifest";
    let manifest = "";

    let mut res = winresource::WindowsResource::new();
    let icon_is_file = fs::metadata(app_icon).map(|m| m.is_file()).unwrap_or(false);
    if icon_is_file {
        res.set_icon(app_icon);
    }
    let manifest_is_file = fs::metadata(manifest).map(|m| m.is_file()).unwrap_or(false);
    if manifest_is_file {
        res.set_manifest_file(manifest);
    }
    // res.set("CompanyName", "Microsoft Corporation")
    //     .set("FileDescription", "WinPE Shell")
    //     .set("FileVersion", "10.0.26100.1 (WinBuild.160101.0800)")
    //     .set("InternalName", "Winpeshl.com")
    //     .set(
    //         "LegalCopyright",
    //         "© Microsoft Corporation. All rights reserved.",
    //     )
    //     .set("OriginalFilename", "WINPESHL.COM")
    //     .set("ProductName", "Microsoft® Windows® Operating System")
    //     .set("ProductVersion", "10.0.26100.1");
    res.compile()?;
    Ok(())
}
