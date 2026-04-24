#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String, Vec};

#[contract]
pub struct JobListing;

#[contractimpl]
impl JobListing {
    /// Post a new job. Employer must provide a stake deposit.
    /// - job_id: Unique identifier for the job
    /// - employer: Address of the employer posting the job
    /// - title: Job title
    /// - salary: Offered salary amount
    /// - deposit: Stake deposit amount (held until job is filled)
    pub fn post_job(
        env: Env,
        job_id: u64,
        employer: Address,
        title: String,
        salary: u64,
        deposit: u64,
    ) {
        employer.require_auth();

        let mut jobs: Map<u64, (Address, String, u64, u64, bool)> = env
            .storage()
            .instance()
            .get(&"jobs")
            .unwrap_or(Map::new(&env));

        if jobs.contains_key(job_id) {
            panic!("Job already exists");
        }

        jobs.set(job_id, (employer, title, salary, deposit, false));
        env.storage().instance().set(&"jobs", &jobs);
    }

    /// Apply to a job. Job seeker records their application.
    /// - job_id: Job identifier to apply to
    /// - seeker: Address of the job seeker
    pub fn apply(env: Env, job_id: u64, seeker: Address) {
        seeker.require_auth();

        let jobs: Map<u64, (Address, String, u64, u64, bool)> = env
            .storage()
            .instance()
            .get(&"jobs")
            .unwrap_or(Map::new(&env));

        let job = jobs.get(job_id).unwrap_or_else(|| panic!("Job not found"));
        let (_, _, _, _, filled) = job;
        if filled {
            panic!("Job is already filled");
        }

        let mut applications: Map<u64, Vec<Address>> = env
            .storage()
            .instance()
            .get(&"applications")
            .unwrap_or(Map::new(&env));

        let mut seekers = applications.get(job_id).unwrap_or(Vec::new(&env));
        seekers.push_back(seeker);
        applications.set(job_id, seekers);
        env.storage().instance().set(&"applications", &applications);
    }

    /// Accept an applicant. Employer accepts a seeker, deposit is returned/transferred.
    /// - job_id: Job identifier
    /// - seeker: Address of the accepted seeker
    pub fn accept_applicant(env: Env, job_id: u64, seeker: Address) {
        let mut jobs: Map<u64, (Address, String, u64, u64, bool)> = env
            .storage()
            .instance()
            .get(&"jobs")
            .unwrap_or(Map::new(&env));

        let job = jobs.get(job_id).unwrap_or_else(|| panic!("Job not found"));
        let (employer, title, salary, deposit, filled) = job;

        if filled {
            panic!("Job is already filled");
        }

        // Mark job as filled
        jobs.set(job_id, (employer, title, salary, deposit, true));
        env.storage().instance().set(&"jobs", &jobs);

        // Note: Actual stake transfer would require integration with
        // Stellar's native asset or token contract. The deposit can serve
        // as a signing bonus transferred to the accepted applicant.
    }

    /// Get job details
    /// Returns: (employer, title, salary, deposit, filled)
    pub fn get_job(
        env: Env,
        job_id: u64,
    ) -> (Address, String, u64, u64, bool) {
        let jobs: Map<u64, (Address, String, u64, u64, bool)> = env
            .storage()
            .instance()
            .get(&"jobs")
            .unwrap_or(Map::new(&env));

        jobs.get(job_id).unwrap_or_else(|| panic!("Job not found"))
    }

    /// Get list of applicants for a job
    pub fn get_applicants(env: Env, job_id: u64) -> Vec<Address> {
        let applications: Map<u64, Vec<Address>> = env
            .storage()
            .instance()
            .get(&"applications")
            .unwrap_or(Map::new(&env));

        applications.get(job_id).unwrap_or(Vec::new(&env))
    }
}
