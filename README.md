```

████████╗███████╗██╗     ███████╗███╗   ███╗███████╗████████╗██████╗ ██╗   ██╗                              
╚══██╔══╝██╔════╝██║     ██╔════╝████╗ ████║██╔════╝╚══██╔══╝██╔══██╗╚██╗ ██╔╝                              
   ██║   █████╗  ██║     █████╗  ██╔████╔██║█████╗     ██║   ██████╔╝ ╚████╔╝                               
   ██║   ██╔══╝  ██║     ██╔══╝  ██║╚██╔╝██║██╔══╝     ██║   ██╔══██╗  ╚██╔╝                                
   ██║   ███████╗███████╗███████╗██║ ╚═╝ ██║███████╗   ██║   ██║  ██║   ██║                                 
   ╚═╝   ╚══════╝╚══════╝╚══════╝╚═╝     ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝                                 
                                                                                                            
██╗    ██╗ ██████╗ ██████╗ ██╗  ██╗██╗      ██████╗  █████╗ ███╗   ██╗    ████████╗███████╗███████╗████████╗
██║    ██║██╔═══██╗██╔══██╗██║ ██╔╝██║     ██╔═══██╗██╔══██╗████╗  ██║    ╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝
██║ █╗ ██║██║   ██║██████╔╝█████╔╝ ██║     ██║   ██║███████║██╔██╗ ██║       ██║   █████╗  ███████╗   ██║   
██║███╗██║██║   ██║██╔══██╗██╔═██╗ ██║     ██║   ██║██╔══██║██║╚██╗██║       ██║   ██╔══╝  ╚════██║   ██║   
╚███╔███╔╝╚██████╔╝██║  ██║██║  ██╗███████╗╚██████╔╝██║  ██║██║ ╚████║       ██║   ███████╗███████║   ██║   
 ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝       ╚═╝   ╚══════╝╚══════╝   ╚═╝   
                                                                                                                 
                          
                                                                                        
```
## Telemetry workloan test
Telemetry workloan test was developed for testing the applications workloan in order to allow to make a comparison between different technologies. Currently there are 4 projects on witch tests are made: Java, Rust, Rust (no ORM) and TypeScript. This project was made using turbo repo and pnpm.

### Architecture
All 4 applications that are tested are installed in 4 different docker containers. Every application connects to the fifth container that contains the DBMS (Postgres).

![architecture](telemetry_workloan_test.png)


### Development stack
 - Java application: Java, Spring Boot, Hibernate, Postgres
 - TS pplication: Typescript, Express, TypeORM, Postgres
 - Rust: Rust, Warp, Sea-ORM, Postgres
 - Rust (no ORM): Rust, Warp, SQLX, Postgres

### Install pnpm
```curl -fsSL https://get.pnpm.io/install.sh | sh -```

### Install packages
```pnpm install```

### Run test
```pnpm run test```

### Tests results are under the directory 
`apps/telemetry_test/reports-html`