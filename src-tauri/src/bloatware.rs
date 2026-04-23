use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct BloatwareApp {
    pub package_name: String,
    pub display_name: String,
    pub publisher: String,
    pub category: String, // "microsoft", "oem_hp", "oem_dell", "oem_lenovo", "oem_asus", "oem_acer", "oem_msi"
    pub installed: bool,
}

/// Get the OEM manufacturer
pub fn detect_manufacturer() -> String {
    let result = crate::powershell::execute(
        "(Get-CimInstance -ClassName Win32_ComputerSystem).Manufacturer"
    );
    if result.success {
        result.stdout.trim().to_string()
    } else {
        "Unknown".to_string()
    }
}

/// Scan for installed bloatware
pub fn scan_bloatware() -> Vec<BloatwareApp> {
    let mut apps = Vec::new();
    let manufacturer = detect_manufacturer().to_lowercase();

    // Microsoft bloatware
    let ms_apps = vec![
        ("Microsoft.BingNews", "Bing News"),
        ("Microsoft.BingWeather", "Bing Weather"),
        ("Microsoft.GetHelp", "Get Help"),
        ("Microsoft.Getstarted", "Tips"),
        ("Microsoft.MicrosoftSolitaireCollection", "Solitaire Collection"),
        ("Microsoft.People", "People"),
        ("Microsoft.PowerAutomate", "Power Automate"),
        ("Microsoft.Todos", "Microsoft To Do"),
        ("Microsoft.WindowsFeedbackHub", "Feedback Hub"),
        ("Microsoft.WindowsMaps", "Maps"),
        ("Microsoft.YourPhone", "Phone Link"),
        ("Microsoft.ZuneMusic", "Groove Music"),
        ("Microsoft.ZuneVideo", "Movies & TV"),
        ("Clipchamp.Clipchamp", "Clipchamp"),
        ("Microsoft.549981C3F5F10", "Cortana"),
        ("Microsoft.MicrosoftOfficeHub", "Office Hub"),
        ("Microsoft.SkypeApp", "Skype"),
        ("Microsoft.Microsoft3DViewer", "3D Viewer"),
        ("Microsoft.MixedReality.Portal", "Mixed Reality Portal"),
        ("Microsoft.OneConnect", "Mobile Plans"),
        ("Microsoft.WindowsAlarms", "Alarms & Clock"),
        ("Microsoft.Xbox.TCUI", "Xbox TCUI"),
        ("Microsoft.XboxGameOverlay", "Xbox Game Overlay"),
        ("Microsoft.XboxGamingOverlay", "Xbox Gaming Overlay"),
        ("Microsoft.XboxIdentityProvider", "Xbox Identity"),
        ("Microsoft.XboxSpeechToTextOverlay", "Xbox Speech"),
    ];

    for (pkg, name) in &ms_apps {
        let installed = !crate::powershell::get_appx_packages(pkg).is_empty();
        apps.push(BloatwareApp {
            package_name: pkg.to_string(),
            display_name: name.to_string(),
            publisher: "Microsoft".to_string(),
            category: "microsoft".to_string(),
            installed,
        });
    }

    // OEM-specific bloatware
    if manufacturer.contains("hp") || manufacturer.contains("hewlett") {
        let hp_apps = vec![
            ("AD2F1837.HPSupportAssistant", "HP Support Assistant"),
            ("AD2F1837.HPAudioSwitch", "HP Audio Switch"),
            ("AD2F1837.HPSystemEventUtility", "HP System Event Utility"),
            ("AD2F1837.HPJumpStarts", "HP JumpStarts"),
            ("AD2F1837.myHP", "myHP"),
            ("AD2F1837.HPPCHardwareDiagnosticsWindows", "HP PC Diagnostics"),
            ("AD2F1837.HPPrivacySettings", "HP Privacy Settings"),
        ];
        for (pkg, name) in &hp_apps {
            let installed = !crate::powershell::get_appx_packages(pkg).is_empty();
            apps.push(BloatwareApp {
                package_name: pkg.to_string(),
                display_name: name.to_string(),
                publisher: "HP".to_string(),
                category: "oem_hp".to_string(),
                installed,
            });
        }
    }

    if manufacturer.contains("dell") {
        let dell_apps = vec![
            ("DellInc.DellSupportAssist", "Dell SupportAssist"),
            ("DellInc.DellDigitalDelivery", "Dell Digital Delivery"),
            ("DellInc.DellCustomerConnect", "Dell Customer Connect"),
            ("DellInc.DellUpdate", "Dell Update"),
            ("DellInc.MyDell", "My Dell"),
        ];
        for (pkg, name) in &dell_apps {
            let installed = !crate::powershell::get_appx_packages(pkg).is_empty();
            apps.push(BloatwareApp {
                package_name: pkg.to_string(),
                display_name: name.to_string(),
                publisher: "Dell".to_string(),
                category: "oem_dell".to_string(),
                installed,
            });
        }
    }

    if manufacturer.contains("lenovo") {
        let lenovo_apps = vec![
            ("E046963F.LenovoCompanion", "Lenovo Vantage"),
            ("E046963F.LenovoNow", "Lenovo Now"),
            ("E046963F.LenovoUtility", "Lenovo Utility"),
            ("E046963F.LenovoID", "Lenovo ID"),
        ];
        for (pkg, name) in &lenovo_apps {
            let installed = !crate::powershell::get_appx_packages(pkg).is_empty();
            apps.push(BloatwareApp {
                package_name: pkg.to_string(),
                display_name: name.to_string(),
                publisher: "Lenovo".to_string(),
                category: "oem_lenovo".to_string(),
                installed,
            });
        }
    }

    if manufacturer.contains("asus") {
        let asus_apps = vec![
            ("B9ECED6F.ArmouryCrate", "ASUS Armoury Crate"),
            ("B9ECED6F.MyASUS", "MyASUS"),
            ("B9ECED6F.ASUSSplendid", "ASUS Splendid"),
        ];
        for (pkg, name) in &asus_apps {
            let installed = !crate::powershell::get_appx_packages(pkg).is_empty();
            apps.push(BloatwareApp {
                package_name: pkg.to_string(),
                display_name: name.to_string(),
                publisher: "ASUS".to_string(),
                category: "oem_asus".to_string(),
                installed,
            });
        }
    }

    if manufacturer.contains("acer") {
        let acer_apps = vec![
            ("AcerIncorporated.AcerCareCenter", "Acer Care Center"),
            ("AcerIncorporated.AcerCollection", "Acer Collection"),
            ("AcerIncorporated.QuickAccess", "Acer Quick Access"),
            ("AcerIncorporated.AcerRegistration", "Acer Registration"),
        ];
        for (pkg, name) in &acer_apps {
            let installed = !crate::powershell::get_appx_packages(pkg).is_empty();
            apps.push(BloatwareApp {
                package_name: pkg.to_string(),
                display_name: name.to_string(),
                publisher: "Acer".to_string(),
                category: "oem_acer".to_string(),
                installed,
            });
        }
    }

    if manufacturer.contains("msi") || manufacturer.contains("micro-star") {
        let msi_apps = vec![
            ("MSI.MSICenter", "MSI Center"),
            ("MSI.DragonCenter", "Dragon Center"),
            ("A-Volute.Nahimic", "Nahimic Audio"),
        ];
        for (pkg, name) in &msi_apps {
            let installed = !crate::powershell::get_appx_packages(pkg).is_empty();
            apps.push(BloatwareApp {
                package_name: pkg.to_string(),
                display_name: name.to_string(),
                publisher: "MSI".to_string(),
                category: "oem_msi".to_string(),
                installed,
            });
        }
    }

    // Filter to only installed apps
    apps.into_iter().filter(|a| a.installed).collect()
}

/// Remove selected bloatware packages
pub fn remove_packages(package_names: Vec<String>) -> Vec<(String, bool)> {
    package_names
        .iter()
        .map(|pkg| {
            let result = crate::powershell::remove_appx_package(pkg);
            (pkg.clone(), result.success || result.stdout.is_empty())
        })
        .collect()
}
