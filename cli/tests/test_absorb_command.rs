    test_env.jj_cmd_ok(&repo_path, &["new"]);
      zsuskuln 3c319ca8 2
      kkmpptxz 6b722c47 1
    Rebased 1 descendant commits.
    Working copy now at: mzvwutvl dfb52c7a (empty) (no description set)
    Parent commit      : zsuskuln 3c319ca8 2
      kkmpptxz 1027ba02 1
    Rebased 2 descendant commits.
    Working copy now at: mzvwutvl 7e01bf33 (empty) (no description set)
    Parent commit      : zsuskuln 669278a7 2
      zsuskuln 660c7cac 2
    Rebased 1 descendant commits.
    Working copy now at: mzvwutvl 39c8fe11 (empty) (no description set)
    Parent commit      : zsuskuln 660c7cac 2
    @  mzvwutvl a0e56cbc (no description set)
    ○  zsuskuln 660c7cac 2
    ○  kkmpptxz 1027ba02 1
    ○    kkmpptxz 1027ba02 1
    │ ○  mzvwutvl hidden 4624004f (no description set)
    │ ○  mzvwutvl hidden dfb52c7a (empty) (no description set)
    ○ │  kkmpptxz hidden 6b722c47 1
    │ ○  mzvwutvl hidden 2342dbe2 (no description set)
    ○    zsuskuln 660c7cac 2
    │ ○  mzvwutvl hidden cb78f902 (no description set)
    │ ○  mzvwutvl hidden 7e01bf33 (empty) (no description set)
    │ ○  mzvwutvl hidden 4624004f (no description set)
    │ ○  mzvwutvl hidden dfb52c7a (empty) (no description set)
    ○ │  zsuskuln hidden 669278a7 2
    ○ │  zsuskuln hidden 3c319ca8 2
    │ ○  mzvwutvl hidden 2342dbe2 (no description set)
      qpvuntsm 9661b868 (conflict) 1
    Rebased 2 descendant commits.
    Working copy now at: zsuskuln f10b6e4e (empty) (no description set)
    Parent commit      : kkmpptxz bed2d032 2
      qpvuntsm 9661b868 (conflict) 1
    @  zsuskuln f10b6e4e (empty) (no description set)
    ○  kkmpptxz bed2d032 2
    ×  qpvuntsm 9661b868 (conflict) 1
    Rebased 1 descendant commits.
    Working copy now at: yqosqzyt 1b10dfa4 (empty) (no description set)
    @  yqosqzyt 1b10dfa4 (empty) (no description set)
    insta::assert_snapshot!(stderr, @r"
    Working copy now at: kkmpptxz 24d6d0f8 (conflict) (no description set)
      kkmpptxz 24d6d0f8 (conflict) (no description set)
    ");
    // discarded because "absorb" isn't a command to squash revisions, but to
    // move hunks.