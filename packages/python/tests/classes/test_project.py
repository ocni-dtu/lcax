import uuid

from lcax import Project, Location, Country, ProjectPhase, SoftwareInfo


def test_project_import():
    from lcax import Project

    assert Project


def test_project_new(assembly):


    project = Project(id=str(uuid.uuid4()), name='Test', description="Test Project",
                      location=Location(country=Country.DNK),
                      life_cycle_modules=[], impact_categories=[],
                      assemblies=[assembly], software_info=SoftwareInfo(lca_software="LCAx"), project_phase=ProjectPhase.OTHER)

    assert project


def test_project_loads(project_file):
    project = Project.loads(project_file.read_text())

    assert project
    assert isinstance(project, Project)


def test_project_dumps(project):
    json_data = project.dumps()

    assert json_data
    assert isinstance(json_data, str)

def test_project_str(project):
    assert str(project) == f"Project: {project.id}"


def test_project_subclass():
    """Test that Project can be subclassed in Python."""
    
    class CustomProject(Project):
        """A custom subclass of Project."""
        
        def custom_method(self):
            return f"Custom Project: {self.name}"
    
    # Create location and software info for the project
    location = Location(country=Country.GBR, city="London")
    software_info = SoftwareInfo(lca_software="Test Software", lca_software_version="1.0")
    
    # Create an instance of the subclass using the parent class constructor
    custom_project = CustomProject(
        name='Custom Project', 
        location=location, 
        project_phase=ProjectPhase.CONCEPT_DESIGN, 
        software_info=software_info,
        life_cycle_modules=[], 
        impact_categories=[], 
        assemblies=[]
    )
    
    # Add custom attributes after creation
    custom_project.custom_field = "project_special"
    
    # Verify it's an instance of both CustomProject and Project
    assert isinstance(custom_project, CustomProject)
    assert isinstance(custom_project, Project)
    
    # Verify the custom field and method work
    assert custom_project.custom_field == "project_special"
    assert custom_project.custom_method() == "Custom Project: Custom Project"
    
    # Verify inherited functionality still works
    assert custom_project.name == "Custom Project"
    assert custom_project.location.city == "London"