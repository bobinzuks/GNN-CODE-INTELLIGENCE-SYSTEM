#!/usr/bin/env python3
"""
MASSIVE REPOSITORY FACTORY - Generates 1000 realistic repositories (5001-6000)
Each with full git history, realistic code, tests, documentation, and evolution
"""

import os
import sys
import json
import random
import subprocess
from pathlib import Path
from datetime import datetime, timedelta
from typing import List, Dict, Any, Tuple
import hashlib

# Repository Templates Configuration
REPO_CATEGORIES = {
    'open_source_clones': {
        'count': 100,
        'templates': [
            'react-clone', 'vue-clone', 'angular-clone', 'django-clone',
            'flask-clone', 'rails-clone', 'spring-boot-clone', 'express-clone',
            'laravel-clone', 'symfony-clone', 'fastapi-clone', 'nextjs-clone',
            'gatsby-clone', 'nuxt-clone', 'nest-clone', 'koa-clone',
            'gin-clone', 'echo-clone', 'actix-clone', 'rocket-clone'
        ]
    },
    'enterprise_ecommerce': {
        'count': 50,
        'templates': ['shopify-clone', 'woocommerce-clone', 'magento-clone', 'bigcommerce-clone']
    },
    'enterprise_crm': {
        'count': 50,
        'templates': ['salesforce-clone', 'hubspot-clone', 'zoho-clone', 'pipedrive-clone']
    },
    'enterprise_erp': {
        'count': 50,
        'templates': ['sap-clone', 'odoo-clone', 'netsuite-clone', 'oracle-erp-clone']
    },
    'enterprise_cms': {
        'count': 50,
        'templates': ['wordpress-clone', 'drupal-clone', 'joomla-clone', 'strapi-clone']
    },
    'enterprise_analytics': {
        'count': 50,
        'templates': ['google-analytics-clone', 'mixpanel-clone', 'amplitude-clone', 'segment-clone']
    },
    'enterprise_bi': {
        'count': 50,
        'templates': ['tableau-clone', 'powerbi-clone', 'looker-clone', 'metabase-clone']
    },
    'enterprise_hr': {
        'count': 50,
        'templates': ['workday-clone', 'bamboo-clone', 'namely-clone', 'gusto-clone']
    },
    'enterprise_pm': {
        'count': 50,
        'templates': ['jira-clone', 'asana-clone', 'monday-clone', 'clickup-clone']
    },
    'microservices_small': {
        'count': 100,
        'templates': ['10-service-ecommerce', '10-service-social', '10-service-booking']
    },
    'microservices_medium': {
        'count': 100,
        'templates': ['50-service-banking', '50-service-healthcare', '50-service-logistics']
    },
    'microservices_large': {
        'count': 50,
        'templates': ['100-service-uber-clone', '100-service-netflix-clone']
    },
    'libraries_web': {
        'count': 100,
        'templates': ['web-framework', 'http-client', 'websocket-lib', 'graphql-lib']
    },
    'libraries_testing': {
        'count': 50,
        'templates': ['test-framework', 'mock-lib', 'assertion-lib', 'e2e-framework']
    },
    'libraries_orm': {
        'count': 50,
        'templates': ['orm-lib', 'query-builder', 'migration-tool', 'schema-validator']
    },
    'cli_tools': {
        'count': 100,
        'templates': ['build-tool', 'package-manager', 'deploy-tool', 'monitor-tool']
    },
    'mobile_ios': {
        'count': 50,
        'templates': ['swiftui-app', 'uikit-app', 'combine-app']
    },
    'mobile_android': {
        'count': 50,
        'templates': ['kotlin-app', 'compose-app', 'coroutines-app']
    },
    'games': {
        'count': 50,
        'templates': ['unity-game', 'godot-game', 'phaser-game']
    },
    'data_engineering': {
        'count': 50,
        'templates': ['etl-pipeline', 'data-warehouse', 'stream-processor', 'ml-pipeline']
    }
}

# Programming Languages Distribution
LANGUAGES = {
    'python': {'weight': 0.30, 'extensions': ['.py']},
    'javascript': {'weight': 0.25, 'extensions': ['.js', '.jsx']},
    'typescript': {'weight': 0.20, 'extensions': ['.ts', '.tsx']},
    'java': {'weight': 0.10, 'extensions': ['.java']},
    'go': {'weight': 0.05, 'extensions': ['.go']},
    'rust': {'weight': 0.03, 'extensions': ['.rs']},
    'ruby': {'weight': 0.03, 'extensions': ['.rb']},
    'php': {'weight': 0.02, 'extensions': ['.php']},
    'c#': {'weight': 0.02, 'extensions': ['.cs']},
}

# Contributor names for realistic commits
CONTRIBUTORS = [
    ('John Smith', 'john.smith@example.com'),
    ('Sarah Johnson', 'sarah.j@example.com'),
    ('Michael Chen', 'mchen@example.com'),
    ('Emily Davis', 'emily.d@example.com'),
    ('David Wilson', 'dwilson@example.com'),
    ('Maria Garcia', 'mgarcia@example.com'),
    ('James Brown', 'jbrown@example.com'),
    ('Lisa Anderson', 'landerson@example.com'),
    ('Robert Taylor', 'rtaylor@example.com'),
    ('Jennifer Martinez', 'jmartinez@example.com'),
    ('William Lee', 'wlee@example.com'),
    ('Jessica White', 'jwhite@example.com'),
    ('Daniel Thomas', 'dthomas@example.com'),
    ('Ashley Moore', 'amoore@example.com'),
    ('Christopher Jackson', 'cjackson@example.com'),
]

class RepoGenerator:
    """Generates realistic repository with full git history"""

    def __init__(self, repo_id: int, category: str, template: str, base_path: str):
        self.repo_id = repo_id
        self.category = category
        self.template = template
        self.base_path = Path(base_path)
        self.repo_name = f"repo-{repo_id:05d}-{template}"
        self.repo_path = self.base_path / self.repo_name
        self.language = self._select_language()
        self.lines_of_code = random.randint(1000, 50000)
        self.num_commits = random.randint(100, 500)
        self.num_contributors = random.randint(1, min(5, len(CONTRIBUTORS)))
        self.contributors = random.sample(CONTRIBUTORS, self.num_contributors)
        self.test_coverage = random.randint(20, 90)

    def _select_language(self) -> str:
        """Select language based on weights"""
        weights = [LANGUAGES[lang]['weight'] for lang in LANGUAGES]
        return random.choices(list(LANGUAGES.keys()), weights=weights)[0]

    def generate(self):
        """Generate complete repository"""
        print(f"[{self.repo_id}] Generating {self.repo_name} ({self.language})...")

        # Create directory
        self.repo_path.mkdir(parents=True, exist_ok=True)
        os.chdir(self.repo_path)

        # Initialize git
        subprocess.run(['git', 'init'], capture_output=True)
        subprocess.run(['git', 'config', 'user.name', self.contributors[0][0]], capture_output=True)
        subprocess.run(['git', 'config', 'user.email', self.contributors[0][1]], capture_output=True)

        # Generate repository structure
        self._generate_structure()

        # Create realistic commit history
        self._create_commit_history()

        print(f"[{self.repo_id}] ✓ Generated {self.repo_name}")

    def _generate_structure(self):
        """Generate project structure based on template"""
        if 'web' in self.template or 'framework' in self.template:
            self._generate_web_structure()
        elif 'microservice' in self.category:
            self._generate_microservice_structure()
        elif 'cli' in self.category:
            self._generate_cli_structure()
        elif 'lib' in self.category or 'lib' in self.template:
            self._generate_library_structure()
        elif 'mobile' in self.category:
            self._generate_mobile_structure()
        elif 'game' in self.category:
            self._generate_game_structure()
        elif 'data' in self.category:
            self._generate_data_structure()
        else:
            self._generate_enterprise_structure()

    def _generate_web_structure(self):
        """Generate web application structure"""
        # Source files
        (self.repo_path / 'src').mkdir()
        (self.repo_path / 'src' / 'components').mkdir()
        (self.repo_path / 'src' / 'pages').mkdir()
        (self.repo_path / 'src' / 'utils').mkdir()
        (self.repo_path / 'src' / 'api').mkdir()
        (self.repo_path / 'src' / 'models').mkdir()
        (self.repo_path / 'src' / 'services').mkdir()

        # Tests
        (self.repo_path / 'tests').mkdir()
        (self.repo_path / 'tests' / 'unit').mkdir()
        (self.repo_path / 'tests' / 'integration').mkdir()
        (self.repo_path / 'tests' / 'e2e').mkdir()

        # Config
        (self.repo_path / 'config').mkdir()

        # Public/Static
        (self.repo_path / 'public').mkdir()
        (self.repo_path / 'static').mkdir()

        self._create_web_files()

    def _generate_microservice_structure(self):
        """Generate microservice architecture"""
        num_services = 10 if 'small' in self.category else (50 if 'medium' in self.category else 100)

        # API Gateway
        (self.repo_path / 'api-gateway').mkdir()
        self._create_service_files(self.repo_path / 'api-gateway', 'gateway')

        # Services
        services = ['auth', 'user', 'product', 'order', 'payment', 'inventory',
                   'notification', 'analytics', 'search', 'recommendation']

        for i in range(min(num_services, 100)):
            service_name = services[i % len(services)] + (f'-{i//len(services)}' if i >= len(services) else '')
            service_path = self.repo_path / 'services' / service_name
            service_path.mkdir(parents=True)
            self._create_service_files(service_path, service_name)

        # Shared
        (self.repo_path / 'shared').mkdir()
        (self.repo_path / 'shared' / 'models').mkdir()
        (self.repo_path / 'shared' / 'utils').mkdir()

        # Infrastructure
        (self.repo_path / 'infrastructure').mkdir()
        self._create_infrastructure_files()

    def _generate_cli_structure(self):
        """Generate CLI tool structure"""
        (self.repo_path / 'cmd').mkdir()
        (self.repo_path / 'internal').mkdir()
        (self.repo_path / 'pkg').mkdir()
        (self.repo_path / 'tests').mkdir()

        self._create_cli_files()

    def _generate_library_structure(self):
        """Generate library structure"""
        (self.repo_path / 'src').mkdir()
        (self.repo_path / 'tests').mkdir()
        (self.repo_path / 'examples').mkdir()
        (self.repo_path / 'docs').mkdir()
        (self.repo_path / 'benchmarks').mkdir()

        self._create_library_files()

    def _generate_mobile_structure(self):
        """Generate mobile app structure"""
        if 'ios' in self.category:
            (self.repo_path / 'Sources').mkdir()
            (self.repo_path / 'Tests').mkdir()
            (self.repo_path / 'Resources').mkdir()
            self._create_ios_files()
        else:
            (self.repo_path / 'app').mkdir()
            (self.repo_path / 'app' / 'src' / 'main' / 'java').mkdir(parents=True)
            (self.repo_path / 'app' / 'src' / 'test' / 'java').mkdir(parents=True)
            (self.repo_path / 'app' / 'src' / 'main' / 'res').mkdir(parents=True)
            self._create_android_files()

    def _generate_game_structure(self):
        """Generate game project structure"""
        (self.repo_path / 'Assets').mkdir()
        (self.repo_path / 'Assets' / 'Scripts').mkdir()
        (self.repo_path / 'Assets' / 'Scenes').mkdir()
        (self.repo_path / 'Assets' / 'Prefabs').mkdir()
        (self.repo_path / 'Assets' / 'Materials').mkdir()
        (self.repo_path / 'Tests').mkdir()

        self._create_game_files()

    def _generate_data_structure(self):
        """Generate data engineering structure"""
        (self.repo_path / 'pipelines').mkdir()
        (self.repo_path / 'transformations').mkdir()
        (self.repo_path / 'models').mkdir()
        (self.repo_path / 'schemas').mkdir()
        (self.repo_path / 'tests').mkdir()
        (self.repo_path / 'config').mkdir()

        self._create_data_files()

    def _generate_enterprise_structure(self):
        """Generate enterprise application structure"""
        (self.repo_path / 'backend').mkdir()
        (self.repo_path / 'backend' / 'src').mkdir()
        (self.repo_path / 'backend' / 'tests').mkdir()

        (self.repo_path / 'frontend').mkdir()
        (self.repo_path / 'frontend' / 'src').mkdir()
        (self.repo_path / 'frontend' / 'tests').mkdir()

        (self.repo_path / 'database').mkdir()
        (self.repo_path / 'database' / 'migrations').mkdir()

        (self.repo_path / 'docs').mkdir()
        (self.repo_path / 'infrastructure').mkdir()

        self._create_enterprise_files()

    def _create_web_files(self):
        """Create web application files"""
        ext = LANGUAGES[self.language]['extensions'][0]

        # Main app file
        self._write_code_file(f'src/app{ext}', self._generate_code(500, 'app_entry'))
        self._write_code_file(f'src/index{ext}', self._generate_code(50, 'index'))

        # Components
        for i in range(random.randint(10, 30)):
            self._write_code_file(f'src/components/Component{i}{ext}',
                                self._generate_code(random.randint(50, 300), 'component'))

        # Pages
        for i in range(random.randint(5, 15)):
            self._write_code_file(f'src/pages/Page{i}{ext}',
                                self._generate_code(random.randint(100, 500), 'page'))

        # Utils
        for i in range(random.randint(5, 15)):
            self._write_code_file(f'src/utils/util{i}{ext}',
                                self._generate_code(random.randint(50, 200), 'util'))

        # API
        for i in range(random.randint(5, 15)):
            self._write_code_file(f'src/api/api{i}{ext}',
                                self._generate_code(random.randint(100, 300), 'api'))

        # Tests
        for i in range(random.randint(20, 60)):
            self._write_code_file(f'tests/unit/test{i}{ext}',
                                self._generate_code(random.randint(50, 200), 'test'))

        self._create_config_files()

    def _create_service_files(self, path: Path, service_name: str):
        """Create microservice files"""
        ext = LANGUAGES[self.language]['extensions'][0]

        (path / 'src').mkdir(exist_ok=True)
        (path / 'tests').mkdir(exist_ok=True)
        (path / 'config').mkdir(exist_ok=True)

        self._write_code_file(f'{path}/src/main{ext}', self._generate_code(200, 'service_main'))
        self._write_code_file(f'{path}/src/handlers{ext}', self._generate_code(300, 'handlers'))
        self._write_code_file(f'{path}/src/models{ext}', self._generate_code(150, 'models'))
        self._write_code_file(f'{path}/src/repository{ext}', self._generate_code(200, 'repository'))

        for i in range(random.randint(5, 15)):
            self._write_code_file(f'{path}/tests/test{i}{ext}',
                                self._generate_code(random.randint(50, 150), 'test'))

    def _create_infrastructure_files(self):
        """Create infrastructure files"""
        # Docker
        self._write_file('docker-compose.yml', self._generate_docker_compose())
        self._write_file('Dockerfile', self._generate_dockerfile())

        # Kubernetes
        k8s_path = self.repo_path / 'infrastructure' / 'k8s'
        k8s_path.mkdir(parents=True, exist_ok=True)
        self._write_file(f'{k8s_path}/deployment.yaml', self._generate_k8s_deployment())
        self._write_file(f'{k8s_path}/service.yaml', self._generate_k8s_service())

        # Terraform
        tf_path = self.repo_path / 'infrastructure' / 'terraform'
        tf_path.mkdir(parents=True, exist_ok=True)
        self._write_file(f'{tf_path}/main.tf', self._generate_terraform())

    def _create_cli_files(self):
        """Create CLI tool files"""
        ext = LANGUAGES[self.language]['extensions'][0]

        self._write_code_file(f'cmd/main{ext}', self._generate_code(300, 'cli_main'))
        self._write_code_file(f'internal/commands{ext}', self._generate_code(500, 'commands'))
        self._write_code_file(f'internal/config{ext}', self._generate_code(150, 'config'))
        self._write_code_file(f'pkg/utils{ext}', self._generate_code(200, 'utils'))

        for i in range(random.randint(10, 30)):
            self._write_code_file(f'tests/test{i}{ext}',
                                self._generate_code(random.randint(50, 150), 'test'))

    def _create_library_files(self):
        """Create library files"""
        ext = LANGUAGES[self.language]['extensions'][0]

        self._write_code_file(f'src/core{ext}', self._generate_code(1000, 'library_core'))

        for i in range(random.randint(10, 30)):
            self._write_code_file(f'src/module{i}{ext}',
                                self._generate_code(random.randint(100, 500), 'module'))

        for i in range(random.randint(20, 60)):
            self._write_code_file(f'tests/test{i}{ext}',
                                self._generate_code(random.randint(50, 200), 'test'))

        for i in range(random.randint(5, 10)):
            self._write_code_file(f'examples/example{i}{ext}',
                                self._generate_code(random.randint(50, 150), 'example'))

    def _create_ios_files(self):
        """Create iOS app files"""
        for i in range(random.randint(10, 30)):
            self._write_code_file(f'Sources/View{i}.swift',
                                self._generate_code(random.randint(50, 200), 'swift_view'))

        for i in range(random.randint(10, 20)):
            self._write_code_file(f'Tests/Test{i}.swift',
                                self._generate_code(random.randint(50, 150), 'swift_test'))

    def _create_android_files(self):
        """Create Android app files"""
        base = 'app/src/main/java/com/example/app'
        for i in range(random.randint(10, 30)):
            self._write_code_file(f'{base}/Activity{i}.kt',
                                self._generate_code(random.randint(50, 200), 'kotlin_activity'))

        test_base = 'app/src/test/java/com/example/app'
        for i in range(random.randint(10, 20)):
            self._write_code_file(f'{test_base}/Test{i}.kt',
                                self._generate_code(random.randint(50, 150), 'kotlin_test'))

    def _create_game_files(self):
        """Create game files"""
        for i in range(random.randint(20, 50)):
            self._write_code_file(f'Assets/Scripts/Script{i}.cs',
                                self._generate_code(random.randint(50, 300), 'game_script'))

    def _create_data_files(self):
        """Create data engineering files"""
        ext = LANGUAGES[self.language]['extensions'][0]

        for i in range(random.randint(5, 15)):
            self._write_code_file(f'pipelines/pipeline{i}{ext}',
                                self._generate_code(random.randint(200, 600), 'pipeline'))

        for i in range(random.randint(10, 20)):
            self._write_code_file(f'transformations/transform{i}{ext}',
                                self._generate_code(random.randint(100, 300), 'transform'))

    def _create_enterprise_files(self):
        """Create enterprise application files"""
        ext = LANGUAGES[self.language]['extensions'][0]

        # Backend
        for i in range(random.randint(20, 50)):
            self._write_code_file(f'backend/src/module{i}{ext}',
                                self._generate_code(random.randint(100, 500), 'backend'))

        # Frontend
        for i in range(random.randint(20, 50)):
            self._write_code_file(f'frontend/src/component{i}{ext}',
                                self._generate_code(random.randint(100, 400), 'frontend'))

        # Database
        for i in range(random.randint(10, 30)):
            self._write_file(f'database/migrations/migration{i:03d}.sql',
                           self._generate_sql_migration())

    def _create_config_files(self):
        """Create configuration files"""
        # Package manager files
        if self.language in ['javascript', 'typescript']:
            self._write_file('package.json', self._generate_package_json())
        elif self.language == 'python':
            self._write_file('setup.py', self._generate_setup_py())
            self._write_file('requirements.txt', self._generate_requirements())
        elif self.language == 'go':
            self._write_file('go.mod', self._generate_go_mod())
        elif self.language == 'rust':
            self._write_file('Cargo.toml', self._generate_cargo_toml())
        elif self.language == 'ruby':
            self._write_file('Gemfile', self._generate_gemfile())
        elif self.language == 'java':
            self._write_file('pom.xml', self._generate_pom_xml())

        # CI/CD
        (self.repo_path / '.github' / 'workflows').mkdir(parents=True, exist_ok=True)
        self._write_file('.github/workflows/ci.yml', self._generate_github_actions())

        # Git
        self._write_file('.gitignore', self._generate_gitignore())

        # README
        self._write_file('README.md', self._generate_readme())

        # License
        self._write_file('LICENSE', self._generate_license())

    def _generate_code(self, lines: int, code_type: str) -> str:
        """Generate realistic code based on language"""
        if self.language == 'python':
            return self._generate_python_code(lines, code_type)
        elif self.language in ['javascript', 'typescript']:
            return self._generate_js_code(lines, code_type)
        elif self.language == 'java':
            return self._generate_java_code(lines, code_type)
        elif self.language == 'go':
            return self._generate_go_code(lines, code_type)
        elif self.language == 'rust':
            return self._generate_rust_code(lines, code_type)
        else:
            return self._generate_generic_code(lines)

    def _generate_python_code(self, lines: int, code_type: str) -> str:
        """Generate Python code"""
        code = []
        code.append('"""')
        code.append(f'Module for {code_type}')
        code.append('"""')
        code.append('')
        code.append('import logging')
        code.append('import typing')
        code.append('from datetime import datetime')
        code.append('')
        code.append('logger = logging.getLogger(__name__)')
        code.append('')

        num_classes = max(1, lines // 50)
        for i in range(num_classes):
            code.append(f'class Component{i}:')
            code.append(f'    """Class for component {i}"""')
            code.append('    ')
            code.append('    def __init__(self, config: dict):')
            code.append('        self.config = config')
            code.append('        self.initialized = False')
            code.append('    ')
            code.append('    def initialize(self):')
            code.append('        """Initialize component"""')
            code.append('        logger.info("Initializing component")')
            code.append('        self.initialized = True')
            code.append('    ')
            code.append('    def process(self, data: typing.Any) -> typing.Any:')
            code.append('        """Process data"""')
            code.append('        if not self.initialized:')
            code.append('            raise RuntimeError("Component not initialized")')
            code.append('        ')
            code.append('        result = self._transform(data)')
            code.append('        return result')
            code.append('    ')
            code.append('    def _transform(self, data: typing.Any) -> typing.Any:')
            code.append('        """Internal transformation logic"""')
            code.append('        # TODO: Implement transformation')
            if random.random() < 0.1:  # 10% chance of bug
                code.append('        return data + None  # BUG: TypeError')
            else:
                code.append('        return data')
            code.append('')

        # Add some functions
        num_functions = max(1, lines // 30)
        for i in range(num_functions):
            code.append(f'def function_{i}(param1: str, param2: int = 0) -> dict:')
            code.append(f'    """Function {i} description"""')
            code.append('    result = {')
            code.append('        "param1": param1,')
            code.append('        "param2": param2,')
            code.append('        "timestamp": datetime.now().isoformat()')
            code.append('    }')
            code.append('    return result')
            code.append('')

        return '\n'.join(code)

    def _generate_js_code(self, lines: int, code_type: str) -> str:
        """Generate JavaScript/TypeScript code"""
        is_ts = self.language == 'typescript'
        code = []

        if is_ts:
            code.append('import { Component } from "./types";')
        else:
            code.append('const Component = require("./component");')

        code.append('')

        num_classes = max(1, lines // 50)
        for i in range(num_classes):
            if is_ts:
                code.append(f'export class Service{i} implements Component {{')
                code.append(f'  private initialized: boolean = false;')
                code.append(f'  private config: Record<string, any>;')
                code.append('')
                code.append(f'  constructor(config: Record<string, any>) {{')
            else:
                code.append(f'class Service{i} {{')
                code.append(f'  constructor(config) {{')

            code.append('    this.config = config;')
            code.append('  }')
            code.append('')
            code.append('  async initialize() {')
            code.append('    console.log("Initializing service");')
            code.append('    this.initialized = true;')
            code.append('  }')
            code.append('')
            code.append('  async process(data) {')
            code.append('    if (!this.initialized) {')
            code.append('      throw new Error("Service not initialized");')
            code.append('    }')
            code.append('    const result = await this.transform(data);')
            code.append('    return result;')
            code.append('  }')
            code.append('')
            code.append('  async transform(data) {')
            if random.random() < 0.1:  # 10% chance of bug
                code.append('    return data.nonexistent.property;  // BUG: Cannot read property')
            else:
                code.append('    return { ...data, processed: true };')
            code.append('  }')
            code.append('}')
            code.append('')

        return '\n'.join(code)

    def _generate_java_code(self, lines: int, code_type: str) -> str:
        """Generate Java code"""
        code = []
        code.append('package com.example.app;')
        code.append('')
        code.append('import java.util.*;')
        code.append('import java.time.LocalDateTime;')
        code.append('')
        code.append('public class Component {')
        code.append('    private Map<String, Object> config;')
        code.append('    private boolean initialized;')
        code.append('')
        code.append('    public Component(Map<String, Object> config) {')
        code.append('        this.config = config;')
        code.append('        this.initialized = false;')
        code.append('    }')
        code.append('')
        code.append('    public void initialize() {')
        code.append('        System.out.println("Initializing component");')
        code.append('        this.initialized = true;')
        code.append('    }')
        code.append('')
        code.append('    public Map<String, Object> process(Object data) {')
        code.append('        if (!initialized) {')
        code.append('            throw new RuntimeException("Component not initialized");')
        code.append('        }')
        code.append('        return transform(data);')
        code.append('    }')
        code.append('')
        code.append('    private Map<String, Object> transform(Object data) {')
        code.append('        Map<String, Object> result = new HashMap<>();')
        code.append('        result.put("data", data);')
        code.append('        result.put("timestamp", LocalDateTime.now());')
        code.append('        return result;')
        code.append('    }')
        code.append('}')

        return '\n'.join(code)

    def _generate_go_code(self, lines: int, code_type: str) -> str:
        """Generate Go code"""
        code = []
        code.append('package main')
        code.append('')
        code.append('import (')
        code.append('    "fmt"')
        code.append('    "time"')
        code.append(')')
        code.append('')
        code.append('type Component struct {')
        code.append('    Config      map[string]interface{}')
        code.append('    Initialized bool')
        code.append('}')
        code.append('')
        code.append('func NewComponent(config map[string]interface{}) *Component {')
        code.append('    return &Component{')
        code.append('        Config:      config,')
        code.append('        Initialized: false,')
        code.append('    }')
        code.append('}')
        code.append('')
        code.append('func (c *Component) Initialize() {')
        code.append('    fmt.Println("Initializing component")')
        code.append('    c.Initialized = true')
        code.append('}')
        code.append('')
        code.append('func (c *Component) Process(data interface{}) (map[string]interface{}, error) {')
        code.append('    if !c.Initialized {')
        code.append('        return nil, fmt.Errorf("component not initialized")')
        code.append('    }')
        code.append('    return c.transform(data)')
        code.append('}')
        code.append('')
        code.append('func (c *Component) transform(data interface{}) (map[string]interface{}, error) {')
        code.append('    result := map[string]interface{}{')
        code.append('        "data":      data,')
        code.append('        "timestamp": time.Now(),')
        code.append('    }')
        code.append('    return result, nil')
        code.append('}')

        return '\n'.join(code)

    def _generate_rust_code(self, lines: int, code_type: str) -> str:
        """Generate Rust code"""
        code = []
        code.append('use std::collections::HashMap;')
        code.append('use chrono::Utc;')
        code.append('')
        code.append('pub struct Component {')
        code.append('    config: HashMap<String, String>,')
        code.append('    initialized: bool,')
        code.append('}')
        code.append('')
        code.append('impl Component {')
        code.append('    pub fn new(config: HashMap<String, String>) -> Self {')
        code.append('        Component {')
        code.append('            config,')
        code.append('            initialized: false,')
        code.append('        }')
        code.append('    }')
        code.append('')
        code.append('    pub fn initialize(&mut self) {')
        code.append('        println!("Initializing component");')
        code.append('        self.initialized = true;')
        code.append('    }')
        code.append('')
        code.append('    pub fn process(&self, data: &str) -> Result<HashMap<String, String>, String> {')
        code.append('        if !self.initialized {')
        code.append('            return Err("Component not initialized".to_string());')
        code.append('        }')
        code.append('        self.transform(data)')
        code.append('    }')
        code.append('')
        code.append('    fn transform(&self, data: &str) -> Result<HashMap<String, String>, String> {')
        code.append('        let mut result = HashMap::new();')
        code.append('        result.insert("data".to_string(), data.to_string());')
        code.append('        result.insert("timestamp".to_string(), Utc::now().to_rfc3339());')
        code.append('        Ok(result)')
        code.append('    }')
        code.append('}')

        return '\n'.join(code)

    def _generate_generic_code(self, lines: int) -> str:
        """Generate generic code"""
        code = []
        for i in range(lines):
            code.append(f'// Line {i}')
        return '\n'.join(code)

    def _write_code_file(self, path: str, content: str):
        """Write code file"""
        self._write_file(path, content)

    def _write_file(self, path: str, content: str):
        """Write file to repository"""
        file_path = self.repo_path / path
        file_path.parent.mkdir(parents=True, exist_ok=True)
        with open(file_path, 'w') as f:
            f.write(content)

    def _generate_package_json(self) -> str:
        """Generate package.json"""
        return json.dumps({
            "name": self.repo_name,
            "version": "1.0.0",
            "description": f"{self.template} project",
            "main": "src/index.js",
            "scripts": {
                "test": "jest",
                "build": "webpack",
                "start": "node src/index.js"
            },
            "dependencies": {
                "express": "^4.18.0",
                "lodash": "^4.17.21"
            },
            "devDependencies": {
                "jest": "^29.0.0",
                "webpack": "^5.75.0"
            }
        }, indent=2)

    def _generate_setup_py(self) -> str:
        """Generate setup.py"""
        return f'''from setuptools import setup, find_packages

setup(
    name="{self.repo_name}",
    version="1.0.0",
    description="{self.template} project",
    packages=find_packages(),
    install_requires=[
        "requests>=2.28.0",
        "pydantic>=1.10.0",
    ],
    python_requires=">=3.8",
)
'''

    def _generate_requirements(self) -> str:
        """Generate requirements.txt"""
        return '''requests>=2.28.0
pydantic>=1.10.0
pytest>=7.2.0
black>=22.10.0
flake8>=5.0.0
'''

    def _generate_go_mod(self) -> str:
        """Generate go.mod"""
        return f'''module github.com/example/{self.repo_name}

go 1.19

require (
    github.com/gin-gonic/gin v1.9.0
    github.com/stretchr/testify v1.8.1
)
'''

    def _generate_cargo_toml(self) -> str:
        """Generate Cargo.toml"""
        return f'''[package]
name = "{self.repo_name}"
version = "1.0.0"
edition = "2021"

[dependencies]
tokio = {{ version = "1.25", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
'''

    def _generate_gemfile(self) -> str:
        """Generate Gemfile"""
        return '''source 'https://rubygems.org'

gem 'rails', '~> 7.0'
gem 'pg', '~> 1.4'

group :development, :test do
  gem 'rspec-rails', '~> 6.0'
end
'''

    def _generate_pom_xml(self) -> str:
        """Generate pom.xml"""
        return f'''<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0">
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>{self.repo_name}</artifactId>
    <version>1.0.0</version>
    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter-web</artifactId>
            <version>3.0.0</version>
        </dependency>
    </dependencies>
</project>
'''

    def _generate_docker_compose(self) -> str:
        """Generate docker-compose.yml"""
        return '''version: '3.8'
services:
  app:
    build: .
    ports:
      - "8000:8000"
    environment:
      - DATABASE_URL=postgresql://user:pass@db:5432/dbname
  db:
    image: postgres:15
    environment:
      - POSTGRES_PASSWORD=pass
'''

    def _generate_dockerfile(self) -> str:
        """Generate Dockerfile"""
        if self.language == 'python':
            return '''FROM python:3.11-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
COPY . .
CMD ["python", "src/main.py"]
'''
        else:
            return '''FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
CMD ["npm", "start"]
'''

    def _generate_k8s_deployment(self) -> str:
        """Generate Kubernetes deployment"""
        return f'''apiVersion: apps/v1
kind: Deployment
metadata:
  name: {self.repo_name}
spec:
  replicas: 3
  selector:
    matchLabels:
      app: {self.repo_name}
  template:
    metadata:
      labels:
        app: {self.repo_name}
    spec:
      containers:
      - name: app
        image: {self.repo_name}:latest
        ports:
        - containerPort: 8000
'''

    def _generate_k8s_service(self) -> str:
        """Generate Kubernetes service"""
        return f'''apiVersion: v1
kind: Service
metadata:
  name: {self.repo_name}
spec:
  selector:
    app: {self.repo_name}
  ports:
  - port: 80
    targetPort: 8000
  type: LoadBalancer
'''

    def _generate_terraform(self) -> str:
        """Generate Terraform config"""
        return '''terraform {
  required_version = ">= 1.0"
}

provider "aws" {
  region = "us-west-2"
}

resource "aws_instance" "app" {
  ami           = "ami-0c55b159cbfafe1f0"
  instance_type = "t2.micro"
}
'''

    def _generate_github_actions(self) -> str:
        """Generate GitHub Actions workflow"""
        return '''name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: |
          npm install
          npm test

  build:
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: npm run build
'''

    def _generate_gitignore(self) -> str:
        """Generate .gitignore"""
        return '''# Dependencies
node_modules/
vendor/
__pycache__/
*.pyc
target/

# Build
dist/
build/
*.so
*.exe

# IDE
.vscode/
.idea/
*.swp

# Environment
.env
.env.local

# Logs
*.log
logs/
'''

    def _generate_readme(self) -> str:
        """Generate README.md"""
        return f'''# {self.repo_name}

{self.template} implementation

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

```bash
# Install dependencies
npm install  # or pip install -r requirements.txt
```

## Usage

```bash
# Run the application
npm start
```

## Testing

```bash
# Run tests
npm test
```

## License

MIT
'''

    def _generate_license(self) -> str:
        """Generate LICENSE"""
        return '''MIT License

Copyright (c) 2024

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction.
'''

    def _generate_sql_migration(self) -> str:
        """Generate SQL migration"""
        return '''-- Migration: Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
'''

    def _create_commit_history(self):
        """Create realistic commit history"""
        start_date = datetime.now() - timedelta(days=365)

        commits = []

        # Initial commit
        commits.append({
            'date': start_date,
            'message': 'Initial commit',
            'author': self.contributors[0],
            'files': ['.gitignore', 'README.md']
        })

        # Feature commits
        all_files = []
        for root, dirs, files in os.walk(self.repo_path):
            for file in files:
                if not file.startswith('.git'):
                    rel_path = os.path.relpath(os.path.join(root, file), self.repo_path)
                    all_files.append(rel_path)

        # Group files into commits
        files_per_commit = len(all_files) // (self.num_commits - 10) if self.num_commits > 10 else 1
        files_per_commit = max(1, files_per_commit)

        commit_types = [
            ('feat', 'Add {}'),
            ('fix', 'Fix bug in {}'),
            ('refactor', 'Refactor {}'),
            ('docs', 'Update documentation for {}'),
            ('test', 'Add tests for {}'),
            ('chore', 'Update dependencies'),
        ]

        current_date = start_date + timedelta(days=1)
        file_index = 0

        while file_index < len(all_files) and len(commits) < self.num_commits:
            commit_type, msg_template = random.choice(commit_types)
            files_in_commit = all_files[file_index:file_index + files_per_commit]

            if files_in_commit:
                component = files_in_commit[0].split('/')[0] if '/' in files_in_commit[0] else 'core'
                message = f"{commit_type}: {msg_template.format(component)}"

                commits.append({
                    'date': current_date,
                    'message': message,
                    'author': random.choice(self.contributors),
                    'files': files_in_commit
                })

                file_index += files_per_commit
                current_date += timedelta(hours=random.randint(1, 48))

        # Create actual git commits
        for commit in commits:
            # Set author
            subprocess.run(['git', 'config', 'user.name', commit['author'][0]], capture_output=True)
            subprocess.run(['git', 'config', 'user.email', commit['author'][1]], capture_output=True)

            # Add files
            for file in commit['files']:
                file_path = self.repo_path / file
                if file_path.exists():
                    subprocess.run(['git', 'add', file], capture_output=True)

            # Commit with date
            env = os.environ.copy()
            env['GIT_AUTHOR_DATE'] = commit['date'].isoformat()
            env['GIT_COMMITTER_DATE'] = commit['date'].isoformat()

            result = subprocess.run(
                ['git', 'commit', '-m', commit['message']],
                capture_output=True,
                env=env
            )

        print(f"  Created {len(commits)} commits")


def main():
    """Main generator function"""
    base_path = Path(__file__).parent

    print("=" * 80)
    print("MASSIVE REPOSITORY FACTORY")
    print("Generating 1000 repositories (5001-6000)")
    print("=" * 80)
    print()

    repo_id = 5001
    total_repos = 0

    for category, config in REPO_CATEGORIES.items():
        count = config['count']
        templates = config['templates']

        print(f"\n[{category.upper()}] Generating {count} repositories...")

        for i in range(count):
            template = templates[i % len(templates)]

            try:
                generator = RepoGenerator(repo_id, category, template, str(base_path))
                generator.generate()
                repo_id += 1
                total_repos += 1

                if total_repos >= 1000:
                    break

            except Exception as e:
                print(f"  ERROR generating repo {repo_id}: {e}")
                repo_id += 1

        if total_repos >= 1000:
            break

    print("\n" + "=" * 80)
    print(f"COMPLETE: Generated {total_repos} repositories")
    print(f"Location: {base_path}")
    print("=" * 80)


if __name__ == '__main__':
    main()
