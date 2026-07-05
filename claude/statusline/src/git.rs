use std::fs;
use std::path::Path;

/// start から親方向に .git を探し、ブランチ名を返す。
/// detached HEAD は短縮 SHA(7 桁)、リポジトリ外は None。
/// worktree(.git が "gitdir: <path>" ファイル)にも対応する。
pub fn current_branch(start: &Path) -> Option<String> {
    let mut dir = Some(start);
    while let Some(d) = dir {
        let dotgit = d.join(".git");
        if dotgit.is_dir() {
            return read_head(&dotgit);
        }
        if dotgit.is_file() {
            let text = fs::read_to_string(&dotgit).ok()?;
            let gitdir = text.strip_prefix("gitdir:")?.trim().to_string();
            return read_head(Path::new(&gitdir));
        }
        dir = d.parent();
    }
    None
}

fn read_head(gitdir: &Path) -> Option<String> {
    let head = fs::read_to_string(gitdir.join("HEAD")).ok()?;
    let head = head.trim();
    if let Some(r) = head.strip_prefix("ref: refs/heads/") {
        return Some(r.to_string());
    }
    Some(head.chars().take(7).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("cs-git-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn reads_branch_from_head() {
        let root = tmp("branch");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        assert_eq!(current_branch(&root).as_deref(), Some("main"));
    }

    #[test]
    fn walks_up_to_repo_root() {
        let root = tmp("nested");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".git/HEAD"), "ref: refs/heads/feature/x\n").unwrap();
        let nested = root.join("a/b/c");
        fs::create_dir_all(&nested).unwrap();
        assert_eq!(current_branch(&nested).as_deref(), Some("feature/x"));
    }

    #[test]
    fn detached_head_returns_short_sha() {
        let root = tmp("detached");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(
            root.join(".git/HEAD"),
            "0123456789abcdef0123456789abcdef01234567\n",
        )
        .unwrap();
        assert_eq!(current_branch(&root).as_deref(), Some("0123456"));
    }

    #[test]
    fn worktree_gitdir_file() {
        let real = tmp("wt-real");
        fs::write(real.join("HEAD"), "ref: refs/heads/wt-branch\n").unwrap();
        let wt = tmp("wt-link");
        fs::write(wt.join(".git"), format!("gitdir: {}\n", real.display())).unwrap();
        assert_eq!(current_branch(&wt).as_deref(), Some("wt-branch"));
    }
}
