//! Access the Repos portion of the GitHub API
imports!();
use client::{GetQueryBuilder, Executor};

new_type!(
    Assignees
    Branches
    Collaborators
    CommitsSha
    CommitsComments
    CommitsStatus
    CommitsStatuses
    Commits
    Repo
    Repos
    Owner
    Pulls
    PullsComments
    PullsCommentsId
    PullsNumber
    PullsNumberComments
    PullsNumberCommits
    PullsNumberFiles
    PullsNumberRequestedReviewers
    PullsNumberMerge
);

from!(
    @Assignees
       => Executor
    @Branches
       => Executor
    @Collaborators
       => Executor
    @CommitsSha
       -> CommitsComments = "comments"
    @CommitsSha
       -> CommitsStatus = "status"
    @CommitsSha
       -> CommitsStatuses = "statuses"
    @CommitsSha
       => Executor
    @CommitsComments
       => Executor
    @CommitsStatus
       => Executor
    @CommitsStatuses
       => Executor
    @Commits
       => CommitsSha
    @Commits
       => Executor
    @GetQueryBuilder
       -> Repos = "repos"
    @Owner
       => Repo
    @Repo
       -> Assignees = "assignees"
    @Repo
       -> Branches = "branches"
    @Repo
       -> Collaborators = "collaborators"
    @Repo
       -> Commits = "commits"
    @Repo
       -> Pulls = "pulls"
    @Repo
       => Executor
    @Repos
       => Owner

    @Pulls
       => Executor
    @Pulls
       -> PullsComments = "comments"
    @PullsComments
       => Executor
    @PullsComments
       => PullsCommentsId
    @PullsCommentsId
       => Executor
    @Pulls
       => PullsNumber
    @PullsNumber
       => Executor
    @PullsNumber
       -> PullsNumberComments = "comments"
    @PullsNumberComments
       => Executor
    @PullsNumber
       -> PullsNumberCommits = "commits"
    @PullsNumberCommits
       => Executor
    @PullsNumber
       -> PullsNumberFiles = "files"
    @PullsNumberFiles
       => Executor
    @PullsNumber
       -> PullsNumberRequestedReviewers = "requested_reviewers"
    @PullsNumberRequestedReviewers
       => Executor
    @PullsNumber
       -> PullsNumberMerge = "merge"
    @PullsNumberMerge
       => Executor

);

impl_macro!(
    @Assignees
        |
        |-> execute
    @Branches
        |
        |-> execute
    @Collaborators
        |
        |-> execute
    @CommitsSha
        |=> comments -> CommitsComments
        |=> status -> CommitsStatus
        |=> statuses -> CommitsStatuses
        |
        |-> execute
    @CommitsComments
        |
        |-> execute
    @CommitsStatus
        |
        |-> execute
    @CommitsStatuses
        |
        |-> execute

    @Commits
        |
        |=> sha -> CommitsSha = sha_str
    @Commits
        |
        |->execute

    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> assignees ->  Assignees
        |=> branches ->  Branches
        |=> collaborators ->  Collaborators
        |=> commits -> Commits
        |=> pulls -> Pulls
        |
        |-> execute

    @Repos
        |
        |=> owner ->  Owner = username_str

    @Pulls
        |=> comments -> PullsComments
        |
        |-> execute
    @Pulls
        |
        |=> number -> PullsNumber = number_str

    @PullsComments
        |
        |=> id -> PullsCommentsId = id_str
    @PullsComments
        |
        |-> execute
    @PullsCommentsId
        |
        |-> execute
    @PullsNumber
        |=> comments -> PullsNumberComments
        |=> commits -> PullsNumberCommits
        |=> files -> PullsNumberFiles
        |=> requested_reviewers -> PullsNumberRequestedReviewers
        |=> merge -> PullsNumberMerge
        |
        |-> execute
    @PullsNumberComments
        |
        |-> execute
    @PullsNumberCommits
        |
        |-> execute
    @PullsNumberFiles
        |
        |-> execute
    @PullsNumberRequestedReviewers
        |
        |-> execute
    @PullsNumberMerge
        |
        |-> execute
);
