# Vimium-TODO

Vimium-TODO is a command-line application designed for efficient task management within IT environments, leveraging Rust for performance, PostgreSQL for robust data storage, and Redis for fast caching of authentication data. It features secure user registration and login functionalities to ensure authorized access. Users can easily add, update, list, and remove tasks through simple console commands, enhancing productivity and organization. The app supports task assignment and completion tracking, allowing teams to manage their projects effectively. Designed for cross-platform availability, Vimium-TODO ensures that tasks are accessible and manageable from any environment.

### [Requriments & QA sheet](https://docs.google.com/spreadsheets/d/1ETFurqxiF-Hr7vgZ0fc1Rm5Ki3MBQSiMvZr4xqjilTM/edit?usp=sharing)

## Business Description

**Company:** SwiftTech Solutions

SwiftTech Solutions is a small IT consulting firm specializing in custom software development, IT support, and cybersecurity solutions for local businesses. The company prides itself on providing tailored solutions that meet the unique needs of each client, ranging from small startups to mid-sized enterprises. SwiftTech Solutions employs a team of software developers, system administrators, and cybersecurity experts who collaborate on various projects. With clients in diverse industries, the company needs an efficient way to manage tasks and projects to ensure timely delivery and maintain high-quality service.

### Business Needs

1. **Task Management:** SwiftTech Solutions requires a robust system for tracking and managing tasks assigned to different team members. This includes creating, updating, and deleting tasks as projects progress.
2. **User Authentication:** To ensure secure access to task management, the system must have simple yet effective user registration and login functionality.
3. **Data Consistency:** The task management data must be reliably stored and quickly accessible. Using Redis for caching authentication data will enhance performance, especially during peak times.
4. **Cross-Platform Availability:** The task management system should be accessible from various environments (e.g., local machines, remote servers) to accommodate the diverse work settings of the team members.
5. **Scalability:** As the company grows, the system should be able to handle an increasing number of tasks and users without a significant drop in performance.
6. **Simplicity and Usability:** The solution should be simple to use, even from a command line interface, ensuring that all team members, regardless of their technical expertise, can use it effectively.

### Potential Growth Areas

1. **Expanded User Roles:** As SwiftTech Solutions grows, the task management system could evolve to include more sophisticated user roles and permissions, enabling better task delegation and oversight.
2. **Integration with Other Tools:** Integrating the system with other project management and communication tools (e.g., Slack, Jira) could streamline workflows and improve productivity.
3. **Advanced Reporting:** Implementing features for generating reports on task progress, team performance, and project timelines could provide valuable insights for management.
4. **Mobile Access:** Developing a mobile-friendly version of the task management system would facilitate access for team members on the go.
5. **Client Access:** Allowing clients to have limited access to view the progress of their projects could improve transparency and client satisfaction.

### Command Line Solution

Given the context, the command line application "Vimium To-Do" will be developed using Rust, PostgreSQL, and Redis. Below is a high-level overview of the features and commands:

1. **User Registration and Login**
   - `register <username> <password>`: Registers a new user.
   - `login <username> <password>`: Logs in an existing user.
   - `logout`: Logs out the current user.

2. **Task Management**
   - `add-todo <task description>`: Adds a new task.
   - `remove-todo <task_id>`: Removes a task by its ID.
   - `list-todos`: Lists all tasks.
   - `update-todo <task_id> <new description>`: Updates the description of an existing task.

3. **Advanced Commands**
   - `assign-task <task_id> <user_id>`: Assigns a task to a specific user.
   - `complete-task <task_id>`: Marks a task as completed.
   - `view-completed`: Lists all completed tasks.

The command line interface will facilitate easy management of tasks and users, supporting SwiftTech Solutions in maintaining productivity and organization across various projects and environments.
