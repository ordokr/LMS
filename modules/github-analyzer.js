/**
 * Analyze GitHub repository data
 */
let Octokit;
try {
  // Try to import the Octokit library if available
  const octokit = require('@octokit/rest');
  Octokit = octokit.Octokit;
} catch (error) {
  // Library not installed, which is fine - GitHub integration will be disabled
  console.log('GitHub integration disabled - @octokit/rest package not installed');
}

const fs = require('fs');
const path = require('path');

class GitHubAnalyzer {
  constructor(metrics, options = {}) {
    this.metrics = metrics;
    
    // Only initialize GitHub client if token is provided AND Octokit is available
    if (options.token && Octokit) {
      this.octokit = new Octokit({ auth: options.token });
    } else {
      this.octokit = null;
    }
    
    this.owner = options.owner || '';
    this.repo = options.repo || '';
    this.cacheDir = options.cacheDir || '.analysis_cache';
  }
  
  /**
   * Analyze GitHub repository
   */
  async analyzeRepository() {
    if (!this.octokit) {
      console.log('GitHub analysis skipped - missing token or package not installed');
      return;
    }
    
    console.log(`Analyzing GitHub repository: ${this.owner}/${this.repo}`);
    
    try {
      // Create metrics structure if it doesn't exist
      if (!this.metrics.github) {
        this.metrics.github = {
          contributors: [],
          commitActivity: [],
          issueStats: {},
          pullRequestStats: {},
          releaseStats: {},
          branchStats: {}
        };
      }
      
      // Fetch data in parallel
      await Promise.all([
        this.fetchContributors(),
        this.fetchCommitActivity(),
        this.fetchIssues(),
        this.fetchPullRequests(),
        this.fetchReleases(),
        this.fetchBranches()
      ]);
      
      // Calculate development velocity
      this.calculateDevelopmentVelocity();
      
      // Cache the results
      this.cacheResults();
      
      console.log('GitHub analysis complete');
      
    } catch (error) {
      console.error('Error analyzing GitHub repository:', error.message);
    }
  }
  
  /**
   * Fetch contributors from GitHub
   */
  async fetchContributors() {
    try {
      const { data } = await this.octokit.repos.listContributors({
        owner: this.owner,
        repo: this.repo
      });
      
      this.metrics.github.contributors = data.map(contributor => ({
        login: contributor.login,
        id: contributor.id,
        contributions: contributor.contributions,
        avatar: contributor.avatar_url,
        url: contributor.html_url
      }));
      
      console.log(`Found ${this.metrics.github.contributors.length} contributors`);
    } catch (error) {
      console.error('Error fetching contributors:', error.message);
    }
  }
  
  /**
   * Fetch commit activity from GitHub
   */
  async fetchCommitActivity() {
    try {
      const { data } = await this.octokit.repos.getCommitActivityStats({
        owner: this.owner,
        repo: this.repo
      });
      
      this.metrics.github.commitActivity = data.map(week => ({
        week: week.week, // Unix timestamp for the start of the week
        additions: week.additions,
        deletions: week.deletions,
        commits: week.total,
        days: week.days // Array of commits per day for the week
      }));
      
      console.log(`Fetched commit activity for ${this.metrics.github.commitActivity.length} weeks`);
    } catch (error) {
      console.error('Error fetching commit activity:', error.message);
    }
  }
  
  /**
   * Fetch issues from GitHub
   */
  async fetchIssues() {
    try {
      // Fetch open issues
      const { data: openIssues } = await this.octokit.issues.listForRepo({
        owner: this.owner,
        repo: this.repo,
        state: 'open',
        per_page: 100
      });
      
      // Fetch closed issues
      const { data: closedIssues } = await this.octokit.issues.listForRepo({
        owner: this.owner,
        repo: this.repo,
        state: 'closed',
        per_page: 100
      });
      
      const allIssues = [...openIssues, ...closedIssues].filter(issue => !issue.pull_request);
      
      // Calculate statistics
      this.metrics.github.issueStats = {
        total: allIssues.length,
        open: openIssues.filter(issue => !issue.pull_request).length,
        closed: closedIssues.filter(issue => !issue.pull_request).length,
        avgTimeToClose: this.calculateAvgTimeToClose(closedIssues),
        byLabel: this.groupByLabels(allIssues),
        byAssignee: this.groupByAssignees(allIssues)
      };
      
      console.log(`Fetched ${allIssues.length} issues`);
    } catch (error) {
      console.error('Error fetching issues:', error.message);
    }
  }
  
  /**
   * Fetch pull requests from GitHub
   */
  async fetchPullRequests() {
    try {
      // Fetch open PRs
      const { data: openPRs } = await this.octokit.pulls.list({
        owner: this.owner,
        repo: this.repo,
        state: 'open',
        per_page: 100
      });
      
      // Fetch closed PRs
      const { data: closedPRs } = await this.octokit.pulls.list({
        owner: this.owner,
        repo: this.repo,
        state: 'closed',
        per_page: 100
      });
      
      const allPRs = [...openPRs, ...closedPRs];
      
      // Calculate PR statistics
      this.metrics.github.pullRequestStats = {
        total: allPRs.length,
        open: openPRs.length,
        closed: closedPRs.length,
        merged: closedPRs.filter(pr => pr.merged).length,
        avgTimeToMerge: this.calculateAvgTimeToMerge(closedPRs),
        byAuthor: this.groupByAuthors(allPRs)
      };
      
      console.log(`Fetched ${allPRs.length} pull requests`);
    } catch (error) {
      console.error('Error fetching pull requests:', error.message);
    }
  }
  
  /**
   * Fetch releases from GitHub
   */
  async fetchReleases() {
    try {
      const { data } = await this.octokit.repos.listReleases({
        owner: this.owner,
        repo: this.repo,
        per_page: 100
      });
      
      this.metrics.github.releaseStats = {
        total: data.length,
        latest: data.length > 0 ? {
          name: data[0].name,
          tag: data[0].tag_name,
          date: data[0].published_at,
          url: data[0].html_url
        } : null,
        frequency: this.calculateReleaseFrequency(data)
      };
      
      console.log(`Fetched ${data.length} releases`);
    } catch (error) {
      console.error('Error fetching releases:', error.message);
    }
  }
  
  /**
   * Fetch branches from GitHub
   */
  async fetchBranches() {
    try {
      const { data } = await this.octokit.repos.listBranches({
        owner: this.owner,
        repo: this.repo,
        per_page: 100
      });
      
      this.metrics.github.branchStats = {
        total: data.length,
        names: data.map(branch => branch.name)
      };
      
      console.log(`Fetched ${data.length} branches`);
    } catch (error) {
      console.error('Error fetching branches:', error.message);
    }
  }
  
  /**
   * Calculate average time to close issues
   */
  calculateAvgTimeToClose(issues) {
    const issuesWithTimes = issues.filter(issue => 
      !issue.pull_request && issue.closed_at && issue.created_at
    );
    
    if (issuesWithTimes.length === 0) return 0;
    
    const totalTimeMs = issuesWithTimes.reduce((sum, issue) => {
      const created = new Date(issue.created_at).getTime();
      const closed = new Date(issue.closed_at).getTime();
      return sum + (closed - created);
    }, 0);
    
    // Return average time in days
    return Math.round(totalTimeMs / issuesWithTimes.length / (1000 * 60 * 60 * 24));
  }
  
  /**
   * Calculate average time to merge PRs
   */
  calculateAvgTimeToMerge(prs) {
    const prsWithTimes = prs.filter(pr => 
      pr.merged && pr.merged_at && pr.created_at
    );
    
    if (prsWithTimes.length === 0) return 0;
    
    const totalTimeMs = prsWithTimes.reduce((sum, pr) => {
      const created = new Date(pr.created_at).getTime();
      const merged = new Date(pr.merged_at).getTime();
      return sum + (merged - created);
    }, 0);
    
    // Return average time in days
    return Math.round(totalTimeMs / prsWithTimes.length / (1000 * 60 * 60 * 24));
  }
  
  /**
   * Group issues by labels
   */
  groupByLabels(issues) {
    const byLabel = {};
    
    issues.forEach(issue => {
      if (issue.labels && issue.labels.length > 0) {
        issue.labels.forEach(label => {
          if (!byLabel[label.name]) {
            byLabel[label.name] = {
              count: 0,
              color: label.color
            };
          }
          byLabel[label.name].count++;
        });
      }
    });
    
    return byLabel;
  }
  
  /**
   * Group issues by assignees
   */
  groupByAssignees(issues) {
    const byAssignee = {};
    
    issues.forEach(issue => {
      if (issue.assignees && issue.assignees.length > 0) {
        issue.assignees.forEach(assignee => {
          if (!byAssignee[assignee.login]) {
            byAssignee[assignee.login] = 0;
          }
          byAssignee[assignee.login]++;
        });
      }
    });
    
    return byAssignee;
  }
  
  /**
   * Group PRs by authors
   */
  groupByAuthors(prs) {
    const byAuthor = {};
    
    prs.forEach(pr => {
      if (pr.user && pr.user.login) {
        if (!byAuthor[pr.user.login]) {
          byAuthor[pr.user.login] = {
            total: 0,
            merged: 0
          };
        }
        byAuthor[pr.user.login].total++;
        if (pr.merged) {
          byAuthor[pr.user.login].merged++;
        }
      }
    });
    
    return byAuthor;
  }
  
  /**
   * Calculate release frequency in days
   */
  calculateReleaseFrequency(releases) {
    if (releases.length < 2) return null;
    
    // Sort releases by date
    const sortedReleases = [...releases].sort((a, b) => 
      new Date(b.published_at).getTime() - new Date(a.published_at).getTime()
    );
    
    let totalDays = 0;
    for (let i = 0; i < sortedReleases.length - 1; i++) {
      const current = new Date(sortedReleases[i].published_at).getTime();
      const previous = new Date(sortedReleases[i + 1].published_at).getTime();
      totalDays += (current - previous) / (1000 * 60 * 60 * 24);
    }
    
    return Math.round(totalDays / (sortedReleases.length - 1));
  }
  
  /**
   * Calculate development velocity from GitHub data
   */
  calculateDevelopmentVelocity() {
    if (!this.metrics.github.commitActivity || this.metrics.github.commitActivity.length === 0) {
      return;
    }
    
    // Look at last 4 weeks of commit activity
    const recentActivity = this.metrics.github.commitActivity.slice(-4);
    
    // Calculate average weekly commits
    const avgWeeklyCommits = recentActivity.reduce((sum, week) => sum + week.commits, 0) / recentActivity.length;
    
    // Calculate PR merge rate
    const prStats = this.metrics.github.pullRequestStats;
    const prMergeRate = prStats && prStats.closed > 0 ? 
      prStats.merged / prStats.closed : 0;
    
    // Calculate issue closure rate
    const issueStats = this.metrics.github.issueStats;
    const issueClosureRate = issueStats && (issueStats.open + issueStats.closed) > 0 ? 
      issueStats.closed / (issueStats.open + issueStats.closed) : 0;
    
    this.metrics.github.velocity = {
      avgWeeklyCommits,
      prMergeRate,
      avgTimeToMergePR: prStats ? prStats.avgTimeToMerge : 0,
      issueClosureRate,
      avgTimeToCloseIssue: issueStats ? issueStats.avgTimeToClose : 0,
      releaseFrequency: this.metrics.github.releaseStats.frequency,
      contributorCount: this.metrics.github.contributors.length,
      activeContributors: this.metrics.github.contributors.filter(c => c.contributions >= 5).length
    };
    
    // Update project velocity data based on GitHub metrics
    if (this.metrics.predictions && this.metrics.predictions.velocityData) {
      // Adjust velocity data based on GitHub activity
      // More commits/PR merges/contributors = faster development
      const velocityModifier = 
        (avgWeeklyCommits > 20 ? 1.2 : avgWeeklyCommits > 10 ? 1.1 : 1.0) *
        (prMergeRate > 0.8 ? 1.15 : prMergeRate > 0.5 ? 1.05 : 1.0) *
        (this.metrics.github.contributors.length > 3 ? 1.1 : 1.0);
      
      this.metrics.predictions.velocityData.models *= velocityModifier;
      this.metrics.predictions.velocityData.apiEndpoints *= velocityModifier;
      this.metrics.predictions.velocityData.uiComponents *= velocityModifier;
      this.metrics.predictions.velocityData.tests *= velocityModifier;
      
      console.log(`Applied GitHub velocity modifier: ${velocityModifier.toFixed(2)}`);
    }
  }
  
  /**
   * Cache GitHub analysis results
   */
  cacheResults() {
    if (!this.cacheDir) return;
    
    try {
      if (!fs.existsSync(this.cacheDir)) {
        fs.mkdirSync(this.cacheDir, { recursive: true });
      }
      
      const cacheFile = path.join(this.cacheDir, 'github_analysis.json');
      fs.writeFileSync(cacheFile, JSON.stringify({
        timestamp: Date.now(),
        data: this.metrics.github
      }, null, 2));
      
      console.log(`GitHub analysis cached to ${cacheFile}`);
    } catch (error) {
      console.error('Error caching GitHub analysis:', error.message);
    }
  }
}

module.exports = GitHubAnalyzer;