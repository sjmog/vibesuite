import { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { PersonasList } from '@/components/personas/persona-list';
import { PersonaDetail } from '@/components/personas/persona-detail';
import { makeRequest } from '@/lib/api';
import type { ProjectPersonaWithTemplate, ApiResponse } from 'shared/types';

export function Team() {
  const { personaId } = useParams();
  const [selectedProject, setSelectedProject] = useState<string | null>(null);
  const [personas, setPersonas] = useState<ProjectPersonaWithTemplate[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Get current project from localStorage or first available project
  useEffect(() => {
    const getSelectedProject = async () => {
      try {
        // Try to get last selected project from localStorage
        const lastProject = localStorage.getItem('selectedProject');
        if (lastProject) {
          setSelectedProject(lastProject);
          return;
        }

        // Fallback: get first available project
        const response = await makeRequest('/api/projects');
        const data: ApiResponse<any[]> = await response.json();
        if (data.success && data.data && data.data.length > 0) {
          setSelectedProject(data.data[0].id);
        }
      } catch (err) {
        setError('Failed to load project');
      }
    };

    getSelectedProject();
  }, []);

  const fetchPersonas = async (projectId: string) => {
    setLoading(true);
    setError(null);
    try {
      const response = await makeRequest(`/api/personas/projects/${projectId}/personas`);
      const data: ApiResponse<ProjectPersonaWithTemplate[]> = await response.json();
      
      if (data.success && data.data) {
        setPersonas(data.data);
      } else {
        setError(data.message || 'Failed to load personas');
      }
    } catch (err) {
      setError('Failed to load personas');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (selectedProject) {
      fetchPersonas(selectedProject);
    }
  }, [selectedProject]);

  const handlePersonaUpdate = () => {
    if (selectedProject) {
      fetchPersonas(selectedProject);
    }
  };

  if (personaId) {
    return (
      <PersonaDetail
        personaId={personaId}
        projectId={selectedProject}
        onBack={() => window.history.back()}
        onUpdate={handlePersonaUpdate}
      />
    );
  }

  return (
    <PersonasList
      projectId={selectedProject}
      personas={personas}
      loading={loading}
      error={error}
      onPersonaUpdate={handlePersonaUpdate}
    />
  );
}