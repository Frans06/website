use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen text-white">
            <main class="relative">
                <HeroSection/>
                <AboutSection/>
                <SkillsSection/>
                <ExperienceSection/>
                <ResearchSection/>
                <EducationSection/>
                <ContactSection/>
            </main>
        </div>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section class="min-h-screen flex items-center justify-center px-4 relative">
            <div class="text-center floating">
                <div class="mb-8">
                    <h1 class="courier text-6xl md:text-8xl font-black text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 via-purple-500 to-pink-500 mb-4">
                        "FRANS RAMIREZ NEYRA"
                    </h1>
                    <div class="h-1 w-32 mx-auto bg-gradient-to-r from-cyan-400 to-purple-500 mb-8"></div>
                    <h2 class="text-2xl md:text-4xl font-light text-gray-300 mb-6">
                        "SOFTWARE ENGINEER"
                    </h2>
                    <p class="text-lg md:text-xl text-gray-400 max-w-3xl mx-auto leading-relaxed">
                        "Expert Fullstack Software Engineer contributing to multiple stage products and companies,
                        focusing on outcomes and product. Full technology context knowledge offering experience 
                        in the full software development lifecycle."
                    </p>
                </div>

                <div class="flex flex-col md:flex-row gap-4 justify-center items-center">
                    <div class="flex items-center gap-2 text-cyan-400">
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                            <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"/>
                            <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"/>
                        </svg>
                        "hello@fransramirez.com"
                    </div>
                    <div class="flex items-center gap-2 text-cyan-400">
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"/>
                        </svg>
                        "Toronto, Ontario"
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn AboutSection() -> impl IntoView {
    view! {
        <section id="about" class="py-20 px-4">
            <div class="max-w-6xl mx-auto">
                <h2 class="courier text-4xl font-bold text-center mb-16 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-cyan-400">
                    "MISSION PROFILE"
                </h2>
                <div class="card-hover rounded-2xl p-8">
                    <p class="text-lg text-gray-300 leading-relaxed text-center max-w-4xl mx-auto">
                        "Agnostic Developer using several tools and languages based on requirements -
                        able to work on optimization or fast development if required. From Frontend to 
                        Infrastructure coverage, I navigate the full spectrum of software engineering 
                        with precision and innovation."
                    </p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SkillsSection() -> impl IntoView {
    view! {
        <section class="py-20 px-4">
            <div class="max-w-6xl mx-auto">
                <h2 class="courier text-4xl font-bold text-center mb-16 text-transparent bg-clip-text bg-gradient-to-r from-pink-400 to-purple-400">
                    "TECHNICAL ARSENAL"
                </h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    <SkillCard title="Languages" skills=vec!["JavaScript/TypeScript", "Python", "Golang", "Rust", "Elixir", "Haskell"]/>
                    <SkillCard title="Protocols & APIs" skills=vec!["XML", "JSON", "SOAP", "REST"]/>
                    <SkillCard title="Databases" skills=vec!["MySQL", "PostgreSQL", "MongoDB", "DynamoDB", "RabbitMQ", "Redis", "Elastic Search"]/>
                    <SkillCard title="Tools" skills=vec!["Vim", "Emacs", "Visual Code", "Docker", "Kubernetes"]/>
                    <SkillCard title="Frontend" skills=vec!["React", "React Native", "Figma", "NextJS", "Remix"]/>
                    <SkillCard title="Infrastructure" skills=vec!["GitHub", "AWS", "GCP", "Phoenix", "Django"]/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SkillCard(title: &'static str, skills: Vec<&'static str>) -> impl IntoView {
    view! {
        <div class="card-hover rounded-xl p-6">
            <h3 class="courier text-xl font-bold mb-4 text-cyan-400">{title}</h3>
            <div class="flex flex-wrap gap-2">
                {skills.into_iter().map(|skill| view! {
                    <span class="px-3 py-1 bg-gradient-to-r from-purple-500/20 to-cyan-500/20 rounded-full text-sm border border-purple-500/30 text-gray-300">
                        {skill}
                    </span>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn ExperienceSection() -> impl IntoView {
    view! {
        <section id="experience" class="py-20 px-4">
            <div class="max-w-6xl mx-auto">
                <h2 class="courier text-4xl font-bold text-center mb-16 text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-pink-400">
                    "MISSION HISTORY"
                </h2>
                <div class="space-y-8">
                    <ExperienceCard
                        company="MyMA"
                        position="Tech Lead"
                        location="New York, NY"
                        period="January 2023 - Present"
                        description="Worked as a Full stack engineer to create an art platform to engage artists and art all in one. Developed a Next.js web app using TypeScript and Node, using AWS infrastructure and all the tooling needed for a progressive App."
                    />
                    <ExperienceCard
                        company="Kanuby"
                        position="Senior Full-stack Software Engineer"
                        location="Mexico City, Mexico"
                        period="November 2022 - March 2023"
                        description="Worked as a Full stack engineer to create a platform to offer Storage services. Developed a Remix.run web app using TypeScript and Node, using Fly.io infrastructure and GitHub CI/CD."
                    />
                    <ExperienceCard
                        company="Vest, Inc."
                        position="Tech Lead"
                        location="Mexico City, Mexico"
                        period="February 2021 - October 2022"
                        description="Developed back-end architecture to connect all services on trading and banking. Built services in Golang for performance optimization and Python for fast development. Built all the IAC in Pulumi and AWS using ECS, ElasticLB, RDS, ElasticCache. Worked on React Native features including Authentication with Cognito and real-time Stock pricing."
                    />
                    <ExperienceCard
                        company="Modus Create"
                        position="Senior Full-stack Software Engineer"
                        location="Mexico City, Mexico"
                        period="August 2020 - February 2021"
                        description="Leveraged Chalice and AWS SDK to build serverless backend and infrastructure for a cloud-based web development platform. Worked with Cloudflare Workers to deploy serverless code globally for Pfizer. Optimized page response times using Cloudflare Cache for Pfizer worldwide pages."
                    />
                    <ExperienceCard
                        company="Chiper"
                        position="Full-stack Software Engineer"
                        location="Mexico City, Mexico"
                        period="September 2019 - March 2020"
                        description="Developed Retail Management System using LoopBack JS, React JS and MySQL for inventory management. Created Operational System for internal processes and bills payment transaction system for POS using KoaJS, Objection and PostgreSQL."
                    />
                    <ExperienceCard
                        company="Credijusto/Covalto Bank"
                        position="Full-stack Software Engineer"
                        location="Mexico City, Mexico"
                        period="March 2019 - September 2019"
                        description="Built environment per branch feature on CI/CD pipeline using GitLab and Kubernetes. Added amortization calculator using Django framework. Created React components for Dash framework dashboards. Maintained Compliance blacklisting system in Golang and worked with OroCRM and Symfony."
                    />
                    <ExperienceCard
                        company="Nure"
                        position="Full-stack and Machine Learning Software Engineer"
                        location="Mexico City, Mexico"
                        period="June 2018 - December 2021"
                        description="Developed Reverse Image Search Engine based on Deep Neural Network (VGG-16 and RESNET) characteristics extraction. Vectorized results and indexed them on Elastic Search. Dockerized solution with Nvidia runtime and orchestration for autoscaling."
                    />
                    <ExperienceCard
                        company="Connus International"
                        position="Software Engineer"
                        location="Mexico City, Mexico"
                        period="March 2017 - September 2018"
                        description="Worked on LPR software integration using C++, training OpenALPR for Mexican car plates. Implemented AWS face recognition for security solutions. Developed IoT monitoring platform using MERN stack. Adapted ML technologies (Detectron, Fast-RCNN, YOLO) with TensorFlow and Caffe for Object Detection."
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn ExperienceCard(
    company: &'static str,
    position: &'static str,
    location: &'static str,
    period: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="card-hover rounded-xl p-8">
            <div class="flex flex-col md:flex-row md:items-center justify-between mb-4">
                <div>
                    <h3 class="courier text-2xl font-bold text-cyan-400 mb-1">{company}</h3>
                    <h4 class="text-lg font-semibold text-purple-400">{position}</h4>
                </div>
                <div class="text-right">
                    <p class="text-gray-400">{period}</p>
                    <p class="text-gray-500 text-sm">{location}</p>
                </div>
            </div>
            <p class="text-gray-300 leading-relaxed">{description}</p>
        </div>
    }
    .into_any()
}

#[component]
fn ResearchSection() -> impl IntoView {
    view! {
        <section class="py-20 px-4">
            <div class="max-w-6xl mx-auto">
                <h2 class="courier text-4xl font-bold text-center mb-16 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-400">
                    "RESEARCH EXPEDITIONS"
                </h2>
                <div class="space-y-8">
                    <div class="card-hover rounded-xl p-8">
                        <h3 class="courier text-2xl font-bold text-cyan-400 mb-2">McGill University</h3>
                        <h4 class="text-lg font-semibold text-purple-400 mb-2">Research Intern</h4>
                        <p class="text-gray-400 mb-4">2016 - Montreal, Canada</p>
                        <p class="text-gray-300 leading-relaxed">
                            "Electronic and mechanical design of drones with Control Engineering implementations.
                            Research in Automatic Maneuvers for Fixed Wings Airplanes."
                        </p>
                    </div>
                    <div class="card-hover rounded-xl p-8">
                        <h3 class="courier text-2xl font-bold text-cyan-400 mb-2">ITESM</h3>
                        <h4 class="text-lg font-semibold text-purple-400 mb-2">Project Investigator</h4>
                        <p class="text-gray-400 mb-4">2016 - Toluca, Mexico</p>
                        <p class="text-gray-300 leading-relaxed">
                            "Investigation on FPGA embedded systems, developing digital filters IIR or FIR and controllers.
                            Contrasting and evaluating performance in analogical and digital filters and controllers."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn EducationSection() -> impl IntoView {
    view! {
        <section class="py-20 px-4">
            <div class="max-w-6xl mx-auto">
                <h2 class="courier text-4xl font-bold text-center mb-16 text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-400">
                    "ACADEMIC ACHIEVEMENTS"
                </h2>
                <div class="card-hover rounded-xl p-8 text-center">
                    <div class="mb-6">
                        <h3 class="courier text-3xl font-bold text-cyan-400 mb-2">Tecnológico de Monterrey</h3>
                        <h4 class="text-xl font-semibold text-purple-400 mb-2">B.S. in Mechatronics & Engineering</h4>
                        <p class="text-gray-400">December 2016</p>
                    </div>
                    <div class="flex flex-col md:flex-row gap-4 justify-center items-center">
                        <div class="pulse-glow px-6 py-3 bg-gradient-to-r from-yellow-500/20 to-orange-500/20 rounded-full border border-yellow-500/30">
                            <span class="text-yellow-400 font-semibold">Double Golden Lamb Award</span>
                        </div>
                        <div class="px-6 py-3 bg-gradient-to-r from-green-500/20 to-cyan-500/20 rounded-full border border-green-500/30">
                            <span class="text-green-400 font-semibold">Overall GPA: 3.85</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ContactSection() -> impl IntoView {
    view! {
        <section id="contact" class="py-20 px-4">
            <div class="max-w-4xl mx-auto text-center">
                <h2 class="courier text-4xl font-bold mb-8 text-transparent bg-clip-text bg-gradient-to-r from-pink-400 to-cyan-400">
                    "INITIATE CONTACT"
                </h2>
                <p class="text-xl text-gray-300 mb-12">
                    "Ready to embark on your next software engineering mission? Let's connect and build something extraordinary together."
                </p>
                <div class="flex flex-col md:flex-row gap-8 justify-center items-center">
                    <a href="mailto:franscaraveli@gmail.com"
                       class="group flex items-center gap-3 px-8 py-4 bg-gradient-to-r from-purple-600 to-cyan-600 rounded-full hover:from-purple-500 hover:to-cyan-500 transition-all duration-300 font-semibold">
                        <svg class="w-6 h-6 group-hover:rotate-12 transition-transform" fill="currentColor" viewBox="0 0 20 20">
                            <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"/>
                            <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"/>
                        </svg>
                        "Send Message"
                    </a>
                    <div class="text-gray-400">
                        "Toronto, Ontario - (437) 661 3660"
                    </div>
                </div>
            </div>
        </section>
    }
}
