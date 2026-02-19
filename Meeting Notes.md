
Meeting summary

Quick recap

The meeting focused on discussing the migration of item store configurations from the older S3-based system to the new Item Store V2. Mahesh presented the changes made to various tools and components, including PRs for item store changes, CI changes, and Jenkins promote jobs. He explained the process of syncing configurations between Item Store V0 and V2, as well as the validation steps required before going to production. The team discussed the use of tags in the new system and how they affect item IDs and searchability. Mahesh demonstrated the migration process using specific examples, including how to fix content mismatches and promote configurations across different environments. The conversation ended with a review of the validation steps and a discussion of potential issues that could arise during the migration process.

Next steps

Mahesh: For each config where validation shows a mismatch between S3 prod and item store V2 (e.g., FFI2), manually fix by syncing the latest version from S3 prod to item store V2 using the migration scripts and promote to all environments as demonstrated.
Mahesh: Remove S3 sync/deployment steps from CI/CD and Jenkins promote jobs for components once all users have migrated to the latest version that uses item store V2, to prevent redundant S3 updates after cutover.
Mahesh: Share the links to all open PRs in item store, tag store, and related repos with the team as mentioned during the meeting.
Mahesh: Ensure the validation script and migration code (in the item store migration repo) are accessible and documented for future reference in case of config drift or issues during cutover.
Summary

Item Store V2 Migration Updates

Mahesh presented updates on item store changes, including PRs for migration to item store V2 and CI updates. He explained the introduction of new filter flags like tags and the removal of dependency on the older S3-based item store. The changes were demonstrated using examples from various tools like vanilla extract, FFI2, and data router, with a focus on transitioning from item store to item store V2. Mahesh also mentioned testing pre-release versions and shared links to open PRs for further review.

Data Router Configuration Updates

Mahesh discussed changes to data router and data quality configurations, focusing on a structural update to multi-document YAML files by adding a root-level "config list" key. They explained that this change requires serializing the files into JSON format and outlined similar updates planned for other tools like FFI and matrix recon. The discussion concluded with a review of GitHub Actions changes for handling feature branches and file path parsing in DMD item storage V2.

Item Store V2 Folder Structure

Mahesh explained the folder structure for item store V2, which includes a config template with folder names for different teams. He described how tags are assigned to configurations and how tenant information is used to determine AWS resources for deployment. Mahesh also discussed the changes made to the item store tooling, which now requires configurations to be in a default folder for S3 deployment. He explained the CI workflow that triggers deployments to item store V2 and S3, and mentioned the addition of a SYNCAS3.yaml workflow for S3 deployments.

S3 Data Quality Configuration Updates

Mahesh explained the process of syncing components and data quality configurations to S3 and item store V2. They described converting new DQ configs to an older format due to JSON limitations, then deploying them to S3. Mahesh clarified that branch names are case-sensitive and mentioned the use of Jenkins jobs for promoting configurations from item store V2 to S3 dev versions.

Jenkins Configuration Promotion Process

Mahesh explained the functionality of Jenkins jobs related to promoting configurations to different environments, particularly focusing on item store V2 and its sync functionality with S3. He described how configurations with specific suffixes (e.g., "_SC") are promoted to S3-only, while others are promoted to both environments. Mahesh also clarified that data engineers use these configurations, and demonstrated how the code handles different promotion scenarios for interfaces like EMR and Spark Submit.

S3 and ComChannel Deployment Strategy

Mahesh explained the deployment strategy for S3 and ComChannel, emphasizing the need to maintain configurations in both V2 and S3 during the migration process. They discussed the importance of syncing data and allowing users to test new versions in the development environment before promoting to production. Mahesh also described the logic behind the promote job, which fetches configurations from the main branch of the item store V2 for QA and prod environments. They mentioned the deployment of an alpha version of the item store package and planned to discuss the item store pre-release in the next segment.

Deployment Process Changes for Item Storage

Mahesh explained the changes made to the deployment process for item storage, including the move from the previous repo to DMD item storage V2 and the modification of file paths. He noted that the default folder is now required for deployment to S3, and the promote and deploy jobs in Jenkins have been disabled. Mahesh also mentioned that using the archived older repo for promotion is not possible due to lack of access.

CI System Tag Application Process

Mahesh explained the process of how tags are applied when files are changed and how the CI system checks for tags.yaml files to add tags during item store API calls. They discussed the importance of having tags in the same location and clarified that the default integration is only for tags, not for S3 or other purposes. Mahesh also mentioned that they are currently migrating configs from S3 to V2, which has a different tax system for canonical-based configs.

Item ID Generation and Migration

Mahesh explained the process of generating item IDs based on tags and discussed the structure of namespaces and contents in item store. They highlighted that tags are crucial for item identification and emphasized the importance of maintaining a consistent tagging system to ensure proper functionality. Mahesh also mentioned the migration of configurations from item store V0 S3 to V2, using an automation process they had previously presented.

ItemStore V2 Configuration Validation Process

Mahesh explained the process of syncing and validating configurations for ItemStore V2, highlighting the use of local files to avoid network bottlenecks. They described a three-step process involving creating a flags table, loading data to ItemStore V2, and validating data quality. Mahesh also introduced a validation script that compares versions from the code and ItemStore V2, emphasizing the need for manual matching in case of discrepancies before production deployment.

Item Store Data Validation Process

Mahesh explained the process of validating item store data, which involves comparing MD5 hashes between production and development environments. They identified a content mismatch issue in item 32 and discussed the steps to fix it by syncing the latest version from S3 production and converting it to JSON format. Mahesh also shared the relevant code repository and files for the item migration process, emphasizing that validation is the key focus.

AWS Credentials and Data Sync

Mahesh configured AWS credentials for a data science non-product account and ran an AWS S3 sync to sync data from an S3 bucket. He then reconfigured prod credentials and validated the changes. Mahesh also converted a file to JSON format and discussed deploying and promoting it.

Item ID Management Process Review

Mahesh discussed the process of creating, updating, and promoting item IDs using various endpoints, including generate item ID and update latest item contents. They encountered issues with mismatches and not-found errors when comparing prod and fraud environments, which they attributed to potential synchronization problems between JSON and YAML formats. The team noted that while the process was tested for one-time efforts, it was not fully end-to-end tested, and they identified FFI2 as a potential area for migration to data management.

AI can make mistakes. Review for accuracy.

Please rate the accuracy of this summary.
