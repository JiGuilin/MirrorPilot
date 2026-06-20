use crate::modules::types::{PackageManager, Source};

/// 获取指定包管理器的内置源列表
pub fn get_builtin_sources(pm: &PackageManager) -> Vec<Source> {
    match pm {
        PackageManager::Npm => vec![
            make_source("npm-official", "npm 官方", "https://registry.npmjs.org/", "npm", "us"),
            make_source("npm-taobao", "淘宝镜像", "https://registry.npmmirror.com/", "npm", "cn"),
            make_source("npm-tencent", "腾讯云", "https://mirrors.tencent.com/npm/", "npm", "cn"),
            make_source("npm-huawei", "华为云", "https://repo.huaweicloud.com/repository/npm/", "npm", "cn"),
            make_source("npm-tuna", "清华镜像", "https://mirrors.tuna.tsinghua.edu.cn/npm/", "npm", "cn"),
            make_source("npm-ustc", "中科大", "https://npmreg.mirrors.ustc.edu.cn/", "npm", "cn"),
        ],
        PackageManager::Yarn => vec![
            make_source("yarn-official", "Yarn 官方", "https://registry.yarnpkg.com/", "yarn", "us"),
            make_source("yarn-taobao", "淘宝镜像", "https://registry.npmmirror.com/", "yarn", "cn"),
            make_source("yarn-tencent", "腾讯云", "https://mirrors.tencent.com/npm/", "yarn", "cn"),
            make_source("yarn-huawei", "华为云", "https://repo.huaweicloud.com/repository/npm/", "yarn", "cn"),
        ],
        PackageManager::Pnpm => vec![
            make_source("pnpm-official", "pnpm 官方", "https://registry.npmjs.org/", "pnpm", "us"),
            make_source("pnpm-taobao", "淘宝镜像", "https://registry.npmmirror.com/", "pnpm", "cn"),
            make_source("pnpm-tencent", "腾讯云", "https://mirrors.tencent.com/npm/", "pnpm", "cn"),
            make_source("pnpm-huawei", "华为云", "https://repo.huaweicloud.com/repository/npm/", "pnpm", "cn"),
        ],
        PackageManager::Pip => vec![
            make_source("pip-official", "PyPI 官方", "https://pypi.org/simple/", "pip", "us"),
            make_source("pip-tuna", "清华镜像", "https://pypi.tuna.tsinghua.edu.cn/simple/", "pip", "cn"),
            make_source("pip-aliyun", "阿里云", "https://mirrors.aliyun.com/pypi/simple/", "pip", "cn"),
            make_source("pip-douban", "豆瓣", "https://pypi.doubanio.com/simple/", "pip", "cn"),
            make_source("pip-huawei", "华为云", "https://repo.huaweicloud.com/repository/pypi/simple/", "pip", "cn"),
            make_source("pip-ustc", "中科大", "https://pypi.mirrors.ustc.edu.cn/simple/", "pip", "cn"),
            make_source("pip-tencent", "腾讯云", "https://mirrors.tencent.com/pypi/simple/", "pip", "cn"),
        ],
        PackageManager::Uv => vec![
            make_source("uv-official", "PyPI 官方", "https://pypi.org/simple/", "uv", "us"),
            make_source("uv-tuna", "清华镜像", "https://pypi.tuna.tsinghua.edu.cn/simple/", "uv", "cn"),
            make_source("uv-aliyun", "阿里云", "https://mirrors.aliyun.com/pypi/simple/", "uv", "cn"),
            make_source("uv-douban", "豆瓣", "https://pypi.doubanio.com/simple/", "uv", "cn"),
            make_source("uv-huawei", "华为云", "https://repo.huaweicloud.com/repository/pypi/simple/", "uv", "cn"),
            make_source("uv-ustc", "中科大", "https://pypi.mirrors.ustc.edu.cn/simple/", "uv", "cn"),
            make_source("uv-tencent", "腾讯云", "https://mirrors.tencent.com/pypi/simple/", "uv", "cn"),
        ],
        PackageManager::Go => vec![
            make_source("go-official", "Go 官方", "https://proxy.golang.org,direct", "go", "us"),
            make_source("go-cn", "Goproxy.cn", "https://goproxy.cn,direct", "go", "cn"),
            make_source("go-aliyun", "阿里云", "https://mirrors.aliyun.com/goproxy/,direct", "go", "cn"),
            make_source("go-qiniu", "七牛云", "https://goproxy.io,direct", "go", "cn"),
        ],
        PackageManager::Maven => vec![
            make_source("maven-central", "Maven Central", "https://repo.maven.apache.org/maven2/", "maven", "us"),
            make_source("maven-aliyun", "阿里云", "https://maven.aliyun.com/repository/central/", "maven", "cn"),
            make_source("maven-huawei", "华为云", "https://repo.huaweicloud.com/repository/maven/", "maven", "cn"),
            make_source("maven-netease", "网易", "https://mirrors.163.com/maven/repository/maven-central/", "maven", "cn"),
            make_source("maven-tuna", "清华镜像", "https://mirrors.tuna.tsinghua.edu.cn/maven/", "maven", "cn"),
        ],
        PackageManager::Gradle => vec![
            make_source("gradle-central", "Maven Central", "https://repo.maven.apache.org/maven2/", "gradle", "us"),
            make_source("gradle-aliyun", "阿里云", "https://maven.aliyun.com/repository/central/", "gradle", "cn"),
            make_source("gradle-huawei", "华为云", "https://repo.huaweicloud.com/repository/maven/", "gradle", "cn"),
        ],
        PackageManager::Docker => vec![
            make_source("docker-hub", "Docker Hub 官方", "https://registry-1.docker.io", "docker", "us"),
            make_source("docker-aliyun", "阿里云", "https://[your-id].mirror.aliyuncs.com", "docker", "cn"),
            make_source("docker-tencent", "腾讯云", "https://mirror.ccs.tencentyun.com", "docker", "cn"),
            make_source("docker-daocloud", "DaoCloud", "https://docker.m.daocloud.io", "docker", "cn"),
            make_source("docker-tuna", "清华镜像", "https://docker.mirrors.tuna.tsinghua.edu.cn", "docker", "cn"),
        ],
        PackageManager::Apt => vec![
            make_source("apt-ubuntu", "Ubuntu 官方", "http://archive.ubuntu.com/ubuntu/", "apt", "us"),
            make_source("apt-aliyun", "阿里云", "https://mirrors.aliyun.com/ubuntu/", "apt", "cn"),
            make_source("apt-tuna", "清华镜像", "https://mirrors.tuna.tsinghua.edu.cn/ubuntu/", "apt", "cn"),
            make_source("apt-ustc", "中科大", "https://mirrors.ustc.edu.cn/ubuntu/", "apt", "cn"),
            make_source("apt-huawei", "华为云", "https://repo.huaweicloud.com/ubuntu/", "apt", "cn"),
        ],
        PackageManager::Yum => vec![
            make_source("yum-centos", "CentOS 官方", "http://mirror.centos.org/centos/", "yum", "us"),
            make_source("yum-aliyun", "阿里云", "https://mirrors.aliyun.com/centos/", "yum", "cn"),
            make_source("yum-tuna", "清华镜像", "https://mirrors.tuna.tsinghua.edu.cn/centos/", "yum", "cn"),
            make_source("yum-ustc", "中科大", "https://mirrors.ustc.edu.cn/centos/", "yum", "cn"),
        ],
        PackageManager::Homebrew => vec![
            make_source("brew-github", "GitHub 官方", "https://github.com/Homebrew/brew", "homebrew", "us"),
            make_source("brew-tuna", "清华镜像", "https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git", "homebrew", "cn"),
            make_source("brew-ustc", "中科大", "https://mirrors.ustc.edu.cn/brew.git", "homebrew", "cn"),
        ],
        PackageManager::Cargo => vec![
            make_source("cargo-crates", "crates.io 官方", "sparse+https://index.crates.io/", "cargo", "us"),
            make_source("cargo-tuna", "清华镜像", "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/", "cargo", "cn"),
            make_source("cargo-ustc", "中科大", "sparse+https://mirrors.ustc.edu.cn/crates.io-index/", "cargo", "cn"),
            make_source("cargo-byte", "字节跳动", "sparse+https://rsproxy.cn/crates.io-index/", "cargo", "cn"),
        ],
        PackageManager::NuGet => vec![
            make_source("nuget-official", "NuGet 官方", "https://api.nuget.org/v3/index.json", "nuget", "us"),
            make_source("nuget-huawei", "华为云", "https://repo.huaweicloud.com/repository/nuget/v3/index.json", "nuget", "cn"),
        ],
        PackageManager::Chocolatey => vec![
            make_source("choco-official", "Chocolatey 官方", "https://chocolatey.org/api/v2/", "chocolatey", "us"),
        ],
        PackageManager::DotNet => vec![
            make_source("dotnet-nuget-official", "NuGet 官方", "https://api.nuget.org/v3/index.json", "dotnet", "us"),
            make_source("dotnet-huawei", "华为云", "https://repo.huaweicloud.com/repository/nuget/v3/index.json", "dotnet", "cn"),
        ],
        PackageManager::Winget => vec![
            make_source("winget-official", "WinGet 官方", "https://cdn.winget.microsoft.com/cache", "winget", "us"),
            make_source("winget-msstore", "Microsoft Store", "https://storeedgefd.dsx.mp.microsoft.com/v9.0", "winget", "us"),
        ],
        PackageManager::Rustup => vec![
            make_source("rustup-official", "Rustup 官方", "https://static.rust-lang.org", "rustup", "us"),
            make_source("rustup-tuna", "清华镜像", "https://mirrors.tuna.tsinghua.edu.cn/rustup", "rustup", "cn"),
            make_source("rustup-ustc", "中科大", "https://mirrors.ustc.edu.cn/rust-static", "rustup", "cn"),
            make_source("rustup-byte", "字节跳动", "https://rsproxy.cn/rustup", "rustup", "cn"),
        ],
    }
}

// ponytail: get_all_builtin_sources() removed, was dead code

fn make_source(id: &str, name: &str, url: &str, pm: &str, region: &str) -> Source {
    Source {
        id: id.to_string(),
        name: name.to_string(),
        url: url.to_string(),
        package_manager: pm.to_string(),
        is_builtin: true,
        is_custom: false,
        region: region.to_string(),
        status: "active".to_string(),
        latency: None,
        speed: None,
        last_tested: None,
    }
}
