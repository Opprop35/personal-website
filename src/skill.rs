#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skill {
    Rust,
    Cpp,
    CSharp,
    LazyVim,
    Git,
    Perforce,
    ArchLinux,
    Bevy,
    Unreal,
    Godot,
    Aseprite,
    Blender,
}

impl Skill {
    pub const ALL: [Self; 12] = [
        Self::Rust,
        Self::Cpp,
        Self::CSharp,
        Self::LazyVim,
        Self::Git,
        Self::Perforce,
        Self::ArchLinux,
        Self::Bevy,
        Self::Unreal,
        Self::Godot,
        Self::Aseprite,
        Self::Blender,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Cpp => "C++",
            Self::CSharp => "C#",
            Self::LazyVim => "LazyVim",
            Self::Git => "Git",
            Self::Perforce => "Perforce",
            Self::ArchLinux => "Arch Linux",
            Self::Bevy => "Bevy",
            Self::Unreal => "Unreal",
            Self::Godot => "Godot",
            Self::Aseprite => "Aseprite",
            Self::Blender => "Blender",
        }
    }

    pub fn image(self) -> &'static str {
        match self {
            Self::Rust => "rust-logo.svg",
            Self::Cpp => "cpp-logo.svg",
            Self::CSharp => "c-sharp.svg",
            Self::LazyVim => "lazyvim.svg",
            Self::Git => "Git-logo.svg",
            Self::Perforce => "perforce.svg",
            Self::ArchLinux => "arch-linux.svg",
            Self::Bevy => "bevy-logo.svg",
            Self::Unreal => "ue-logo.svg",
            Self::Godot => "Godot-logo.svg",
            Self::Aseprite => "aseprite.svg",
            Self::Blender => "blender.svg",
        }
    }

    pub fn alt(self) -> &'static str {
        match self {
            Self::Rust => "Rust logo",
            Self::Cpp => "C++ logo",
            Self::CSharp => "C# logo",
            Self::LazyVim => "LazyVim logo",
            Self::Git => "Git logo",
            Self::Perforce => "Perforce logo",
            Self::ArchLinux => "Arch Linux logo",
            Self::Bevy => "Bevy logo",
            Self::Unreal => "Unreal logo",
            Self::Godot => "Godot logo",
            Self::Aseprite => "Aseprite logo",
            Self::Blender => "Blender logo",
        }
    }

    pub const fn url(self) -> &'static str {
        match self {
            Self::Rust => "https://www.rust-lang.org",
            Self::Cpp => "https://isocpp.org",
            Self::CSharp => "https://dotnet.microsoft.com/en-us/languages/csharp",
            Self::LazyVim => "https://www.lazyvim.org",
            Self::Git => "https://git-scm.com",
            Self::Perforce => "https://www.perforce.com",
            Self::ArchLinux => "https://archlinux.org",
            Self::Bevy => "https://bevyengine.org",
            Self::Unreal => "https://www.unrealengine.com",
            Self::Godot => "https://godotengine.org",
            Self::Aseprite => "https://www.aseprite.org",
            Self::Blender => "https://www.blender.org",
        }
    }

    pub fn years(self) -> f32 {
        match self {
            Self::Rust => 1.5,
            Self::Cpp => 4.0,
            Self::CSharp => 4.0,
            Self::LazyVim => 1.0,
            Self::Git => 8.0,
            Self::Perforce => 5.0,
            Self::ArchLinux => 1.0,
            Self::Bevy => 1.0,
            Self::Unreal => 6.0,
            Self::Godot => 4.0,
            Self::Aseprite => 1.0,
            Self::Blender => 0.5,
        }
    }
}
