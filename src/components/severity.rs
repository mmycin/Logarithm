/// Numeric rank for severity-based filtering.
pub fn severity_rank(s: &str) -> u8 {
    match s.to_ascii_uppercase().as_str() {
        "TRACE"            => 0,
        "DEBUG"            => 1,
        "INFO" | "SUCCESS" => 2,
        "WARN" | "WARNING" => 3,
        "ERROR"            => 4,
        "FATAL"            => 5,
        _                  => 99,
    }
}

/// Level pills: (filter_value, display_label, hex_color)
pub const LEVEL_PILLS: &[(&str, &str, &str)] = &[
    ("all",     "All",     "#7c9dff"),
    ("info",    "Info",    "#60a5fa"),
    ("warn",    "Warn",    "#f59e0b"),
    ("error",   "Error",   "#f87171"),
    ("debug",   "Debug",   "#94a3b8"),
    ("fatal",   "Fatal",   "#e879f9"),
    ("trace",   "Trace",   "#22d3ee"),
    ("success", "Success", "#34d399"),
];

/// Min-severity options: (value, label, color)
pub const MIN_SEV: &[(&str, &str, &str)] = &[
    ("all",   "Any",    "#7c9dff"),
    ("debug", "Debug+", "#94a3b8"),
    ("info",  "Info+",  "#60a5fa"),
    ("warn",  "Warn+",  "#f59e0b"),
    ("error", "Error+", "#f87171"),
    ("fatal", "Fatal",  "#e879f9"),
];

/// Level → (text, bg, border, left-accent) colors for dark/light
pub fn level_colors(level: &str, dark: bool) -> (&'static str, &'static str, &'static str, &'static str) {
    match level {
        "INFO"             => if dark { ("#93c5fd","#93c5fd14","#93c5fd35","transparent") } else { ("#2563eb","#2563eb0e","#2563eb30","transparent") },
        "DEBUG"            => if dark { ("#94a3b8","#94a3b810","#94a3b825","transparent") } else { ("#64748b","#64748b0a","#64748b20","transparent") },
        "TRACE"            => if dark { ("#67e8f9","#67e8f910","#67e8f925","transparent") } else { ("#0891b2","#0891b20a","#0891b220","transparent") },
        "SUCCESS"          => if dark { ("#6ee7b7","#6ee7b714","#6ee7b730","transparent") } else { ("#059669","#0596690e","#05966930","transparent") },
        "WARN"|"WARNING"   => if dark { ("#fcd34d","#fcd34d12","#fcd34d35","#f59e0b")    } else { ("#d97706","#d977060e","#d9770630","#f59e0b")    },
        "ERROR"            => if dark { ("#fca5a5","#fca5a514","#fca5a535","#ef4444")    } else { ("#dc2626","#dc26260e","#dc262630","#ef4444")    },
        "FATAL"            => if dark { ("#f0abfc","#f0abfc14","#f0abfc35","#d946ef")    } else { ("#9333ea","#9333ea0e","#9333ea30","#d946ef")    },
        _                  => if dark { ("#8b92a8","#00000000","#00000000","transparent") } else { ("#6b7280","#00000000","#00000000","transparent") },
    }
}
