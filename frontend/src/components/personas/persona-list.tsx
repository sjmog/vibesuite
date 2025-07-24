import { useState } from 'react';
import { Plus, Upload, AlertCircle } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { PersonaCard } from './persona-card';
import { PersonaForm } from './persona-form';
import { makeRequest } from '@/lib/api';
import type { ProjectPersonaWithTemplate, ProjectPersona, ApiResponse } from 'shared/types';

interface PersonasListProps {
  projectId: string | null;
  personas: ProjectPersonaWithTemplate[];
  loading: boolean;
  error: string | null;
  onPersonaUpdate: () => void;
}

export function PersonasList({ 
  projectId, 
  personas, 
  loading, 
  error, 
  onPersonaUpdate 
}: PersonasListProps) {
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [importing, setImporting] = useState(false);

  const handleImportDefaults = async () => {
    if (!projectId) return;
    
    setImporting(true);
    try {
      const response = await makeRequest(`/api/personas/projects/${projectId}/personas/import-defaults`, {
        method: 'POST',
      });
      const data: ApiResponse<ProjectPersona[]> = await response.json();
      
      if (data.success) {
        onPersonaUpdate();
      } else {
        console.error('Failed to import default personas:', data.message);
      }
    } catch (err) {
      console.error('Failed to import default personas:', err);
    } finally {
      setImporting(false);
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto p-6">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto"></div>
            <p className="mt-2 text-muted-foreground">Loading team...</p>
          </div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto p-6">
        <div className="flex items-center justify-center min-h-[400px]">
          <Card className="w-full max-w-md">
            <CardContent className="pt-6">
              <div className="flex items-center space-x-2 text-destructive">
                <AlertCircle className="h-4 w-4" />
                <p>{error}</p>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    );
  }

  if (!projectId) {
    return (
      <div className="container mx-auto p-6">
        <div className="flex items-center justify-center min-h-[400px]">
          <Card className="w-full max-w-md">
            <CardContent className="pt-6">
              <p className="text-muted-foreground text-center">
                No project selected. Please select a project to manage personas.
              </p>
            </CardContent>
          </Card>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">Team</h1>
          <p className="text-muted-foreground">
            Manage AI personas for your project team
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <Button 
            variant="outline" 
            onClick={handleImportDefaults}
            disabled={importing}
          >
            <Upload className="mr-2 h-4 w-4" />
            {importing ? 'Importing...' : 'Import Defaults'}
          </Button>
          <Button onClick={() => setShowCreateDialog(true)}>
            <Plus className="mr-2 h-4 w-4" />
            Add Persona
          </Button>
        </div>
      </div>

      {personas.length === 0 ? (
        <Card className="w-full">
          <CardHeader className="text-center">
            <CardTitle>No personas yet</CardTitle>
            <CardDescription>
              Get started by importing default personas or creating custom ones
            </CardDescription>
          </CardHeader>
          <CardContent className="text-center space-y-2">
            <Button onClick={handleImportDefaults} disabled={importing}>
              <Upload className="mr-2 h-4 w-4" />
              {importing ? 'Importing...' : 'Import Default Personas'}
            </Button>
            <p className="text-muted-foreground text-sm">or</p>
            <Button variant="outline" onClick={() => setShowCreateDialog(true)}>
              <Plus className="mr-2 h-4 w-4" />
              Create Custom Persona
            </Button>
          </CardContent>
        </Card>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {personas.map((persona) => (
            <PersonaCard 
              key={persona.id} 
              persona={persona} 
              onUpdate={onPersonaUpdate}
            />
          ))}
        </div>
      )}

      <PersonaForm
        open={showCreateDialog}
        onOpenChange={setShowCreateDialog}
        projectId={projectId}
        onSuccess={onPersonaUpdate}
      />
    </div>
  );
}