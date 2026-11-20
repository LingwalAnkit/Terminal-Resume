//! Resume content, taken from the resume PDF. Edit this file to update the resume.
//!
//! Anything the PDF does not contain (a headline, a portfolio link, links per project)
//! is left out rather than guessed. Fields marked "draft" were written from the PDF
//! and should be checked.

pub struct Resume {
    pub name: &'static str,
    /// Shown under the name. Draft: the PDF has no headline, so this comes from the degree.
    pub role: &'static str,
    /// Draft: written from the projects in the PDF. Edit to your own wording.
    pub summary: &'static str,
    pub achievements: &'static [&'static str],
    pub skills: &'static [SkillGroup],
    pub projects: &'static [Project],
    pub experience: &'static [Job],
    pub education: &'static [Education],
    pub contact: Contact,
}

pub struct SkillGroup {
    pub label: &'static str,
    pub items: &'static [&'static str],
}

pub struct Project {
    pub name: &'static str,
    pub stack: &'static [&'static str],
    pub highlights: &'static [&'static str],
}

pub struct Job {
    pub title: &'static str,
    pub company: &'static str,
    pub location: &'static str,
    pub period: &'static str,
    pub highlights: &'static [&'static str],
}

pub struct Education {
    pub institution: &'static str,
    pub location: &'static str,
    pub degree: &'static str,
    pub cgpa: &'static str,
    pub period: &'static str,
}

pub struct Contact {
    pub email: &'static str,
    pub phone: &'static str,
    pub linkedin: &'static str,
    pub github: &'static str,
}

pub const RESUME: Resume = Resume {
    name: "Ankit Singh Lingwal",
    role: "B.Tech student, Industrial Internet of Things",
    summary: "Rust developer building async networking and backend systems: a QUIC RPC framework, a PostgreSQL-backed job queue, and Solidity smart contracts.",
    achievements: &[
        "Smart India Hackathon: Grand Finalist, 2024 and 2025",
        "Winner of two hackathons: \u{201C}HackSmart\u{201D} and \u{201C}Hack Summit\u{201D} organized by GeekRoom",
    ],
    skills: &[
        SkillGroup {
            label: "Languages",
            items: &["Rust", "Solidity", "JavaScript", "TypeScript", "HTML/CSS"],
        },
        SkillGroup {
            label: "Rust & Systems",
            items: &[
                "Tokio (async)",
                "Axum",
                "Quinn (QUIC)",
                "Rustls (mTLS)",
                "Tokio-util codecs",
                "concurrency",
                "proptest",
            ],
        },
        SkillGroup {
            label: "Web & Blockchain",
            items: &[
                "React", "Node.js", "Next.js", "Foundry", "Hardhat", "Web3.js",
            ],
        },
        SkillGroup {
            label: "Databases & Tools",
            items: &[
                "PostgreSQL",
                "MongoDB",
                "Git",
                "Google Cloud Platform",
                "VS Code",
            ],
        },
    ],
    projects: &[
        Project {
            name: "RPC Framework over QUIC",
            stack: &["Rust", "Tokio", "Quinn", "Rustls"],
            highlights: &[
                "Built a from-scratch asynchronous RPC framework over QUIC (quinn) with a custom framed binary wire protocol, a hand-written Tokio codec (partial reads, backpressure, 16 MB cap), and mutual TLS (self-issued CA, client-certificate verification, ALPN).",
                "Implemented client-to-server deadline propagation and cooperative cancellation in both directions (dropped calls reset streams and abort in-flight server work), with typed gRPC-style error codes.",
                "Built graceful shutdown that drains in-flight requests within a bounded timeout, verified with property-based tests (proptest); currently extending the system toward Raft-based replication and BFT/DAG consensus.",
            ],
        },
        Project {
            name: "Rust Background Job Queue",
            stack: &["Rust", "Axum", "Tokio", "PostgreSQL"],
            highlights: &[
                "Built a persistent background job queue with an Axum API, PostgreSQL-backed job storage, and a Tokio worker pool for reliable asynchronous task execution.",
                "Implemented configurable retries with exponential backoff, job state transitions, failure handling, and dead-letter semantics for jobs exceeding maximum attempts.",
                "Added concurrent worker coordination, graceful shutdown, job recovery after process restarts, and atomic job claiming to prevent duplicate execution.",
            ],
        },
        Project {
            name: "Cross-Chain Fund Transfer Bridge",
            stack: &["Solidity", "Foundry", "Node.js", "Web3.js"],
            highlights: &[
                "Developed a cross-chain bridge contract for fund transfers between EVM-based blockchain networks.",
                "Built smart contracts in Solidity with the Foundry framework for efficient cross-chain operations.",
                "Integrated Web3.js to bridge decentralized blockchain functionality with traditional web2 interfaces.",
            ],
        },
    ],
    experience: &[
        Job {
            title: "Plugin Developer Intern",
            company: "Waysorted",
            location: "New Delhi, IN",
            period: "Apr 2025 \u{2013} Jan 2026",
            highlights: &[
                "Designed and developed custom Figma plugins with both UI and backend components, ensuring smooth integration within the Figma environment.",
                "Explored the Figma Plugin API and improved user experience through intuitive plugin features.",
            ],
        },
        Job {
            title: "Full Stack Web Development Intern",
            company: "IPU IIF",
            location: "New Delhi, IN",
            period: "Jan. 2025 \u{2013} Apr 2025",
            highlights: &[
                "Led end-to-end development of scalable web applications using Next.js, TypeScript, and MongoDB.",
                "Designed backend architectures and collaborated with cross-functional teams to improve application performance.",
            ],
        },
        Job {
            title: "MERN Developer Intern",
            // Spelled as in the PDF. Check whether this is meant to be "Authority".
            company: "AutorityMartMedia",
            location: "New Delhi, IN",
            period: "Feb 2025 \u{2013} Apr 2025",
            highlights: &[
                "Built a MailVerify tool for verifying email addresses, including a user-friendly interface and plugin integrations.",
                "Integrated third-party services (Zapier, Mailchimp, Moosend) and led frontend development with backend integration.",
            ],
        },
    ],
    education: &[Education {
        institution: "Guru Gobind Singh Indraprastha University",
        location: "New Delhi, IN",
        degree: "B.Tech in Industrial Internet of Things",
        cgpa: "8.24",
        period: "Sep. 2023 \u{2013} Jun 2027",
    }],
    contact: Contact {
        email: "ankitlingwal090@gmail.com",
        phone: "+91 6397385537",
        linkedin: "linkedin.com/in/ankit-singh-lingwal",
        github: "github.com/LingwalAnkit",
    },
};
