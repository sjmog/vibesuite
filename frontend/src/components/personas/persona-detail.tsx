import { useState, useEffect } from 'react';
import { ArrowLeft, Edit, Calendar, Activity } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { PersonaForm } from './persona-form';
import { ActivityHistory } from './activity-history';
import { makeRequest } from '@/lib/api';
import type { ProjectPersonaWithTemplate, ApiResponse } from 'shared/types';

interface PersonaDetailProps {
  personaId: string;
  projectId: string | null;
  onBack: () => void;
  onUpdate: () => void;
}

export function PersonaDetail({ personaId, projectId, onBack, onUpdate }: PersonaDetailProps) {
  const [persona, setPersona] = useState<ProjectPersonaWithTemplate | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showEditDialog, setShowEditDialog] = useState(false);

  useEffect(() => {
    const fetchPersona = async () => {
      if (!projectId) return;
      
      setLoading(true);
      try {
        // We need to fetch from the project personas list since there's no single persona endpoint
        const response = await makeRequest(`/api/personas/projects/${projectId}/personas`);
        const data: ApiResponse<ProjectPersonaWithTemplate[]> = await response.json();
        
        if (data.success && data.data) {
          const found = data.data.find(p => p.id === personaId);
          if (found) {
            setPersona(found);
          } else {
            setError('Persona not found');
          }
        } else {
          setError(data.message || 'Failed to load persona');
        }
      } catch (err) {
        setError('Failed to load persona');
      } finally {
        setLoading(false);
      }
    };

    fetchPersona();
  }, [personaId, projectId]);

  const handleUpdate = () => {
    onUpdate();
    // Refresh persona data
    if (projectId) {
      const fetchPersona = async () => {
        try {
          const response = await makeRequest(`/api/personas/projects/${projectId}/personas`);
          const data: ApiResponse<ProjectPersonaWithTemplate[]> = await response.json();
          
          if (data.success && data.data) {
            const found = data.data.find(p => p.id === personaId);
            if (found) {
              setPersona(found);
            }
          }
        } catch (err) {
          console.error('Failed to refresh persona:', err);
        }
      };
      fetchPersona();
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto p-6">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto"></div>
            <p className="mt-2 text-muted-foreground">Loading persona...</p>
          </div>
        </div>
      </div>
    );
  }

  if (error || !persona) {
    return (
      <div className="container mx-auto p-6">
        <div className="flex items-center justify-center min-h-[400px]">
          <Card className="w-full max-w-md">
            <CardContent className="pt-6">
              <p className="text-center text-destructive">
                {error || 'Persona not found'}
              </p>
              <div className="flex justify-center mt-4">
                <Button variant="outline" onClick={onBack}>
                  <ArrowLeft className="mr-2 h-4 w-4" />
                  Back to Team
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    );
  }

  const displayName = persona.custom_name || persona.template_name;
  const roleType = persona.template_role_type.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());

  const getScoreColor = (score: number) => {
    if (score < 0) return 'text-destructive';
    if (score < 10) return 'text-muted-foreground';
    if (score < 25) return 'text-blue-600';
    if (score < 100) return 'text-green-600';
    return 'text-yellow-600';
  };

  const getScoreLabel = (score: number, type: 'P' | 'Q') => {
    if (score < 0) return 'Issues';
    if (score < 10) return 'Standard';
    if (score < 25) return 'Senior';
    if (score < 100) return 'Elite';
    return type === 'P' ? 'Ultra Mega' : 'Master';
  };

  return (
    <div className="container mx-auto p-6">
      <div className="mb-6">
        <Button variant="ghost" onClick={onBack} className="mb-4">
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Team
        </Button>
        
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-3xl font-bold tracking-tight">{displayName}</h1>
            <div className="flex items-center space-x-2 mt-2">
              <Badge variant="outline">{roleType}</Badge>
              <Badge variant={persona.is_active ? 'default' : 'secondary'}>
                {persona.is_active ? 'Active' : 'Inactive'}
              </Badge>
            </div>
            <p className="text-muted-foreground mt-2">
              {persona.template_description}
            </p>
          </div>
          <Button onClick={() => setShowEditDialog(true)}>
            <Edit className="mr-2 h-4 w-4" />
            Edit Persona
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">Professionalism Score</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              <span className={getScoreColor(persona.professionalism_score)}>
                {Math.round(persona.professionalism_score * 10) / 10}
              </span>
            </div>
            <p className="text-xs text-muted-foreground mt-1">
              {getScoreLabel(persona.professionalism_score, 'P')}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">Quality Score</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              <span className={getScoreColor(persona.quality_score)}>
                {Math.round(persona.quality_score * 10) / 10}
              </span>
            </div>
            <p className="text-xs text-muted-foreground mt-1">
              {getScoreLabel(persona.quality_score, 'Q')}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-medium">Member Since</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {new Date(persona.created_at).toLocaleDateString()}
            </div>
            <p className="text-xs text-muted-foreground mt-1">
              Created {new Date(persona.created_at).toLocaleDateString()}
            </p>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardContent className="p-0">
          <Tabs defaultValue="details" className="w-full">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="details">Details</TabsTrigger>
              <TabsTrigger value="activity">
                <Activity className="mr-2 h-4 w-4" />
                Activity
              </TabsTrigger>
              <TabsTrigger value="history">
                <Calendar className="mr-2 h-4 w-4" />
                Action History
              </TabsTrigger>
            </TabsList>
            
            <TabsContent value="details" className="p-6">
              <div className="space-y-4">
                <div>
                  <h3 className="font-semibold mb-2">Template Information</h3>
                  <div className="space-y-2 text-sm">
                    <div>
                      <span className="font-medium">Name:</span> {persona.template_name}
                    </div>
                    <div>
                      <span className="font-medium">Role:</span> {roleType}
                    </div>
                    <div>
                      <span className="font-medium">Description:</span> {persona.template_description}
                    </div>
                  </div>
                </div>

                {persona.custom_name && (
                  <div>
                    <h3 className="font-semibold mb-2">Custom Configuration</h3>
                    <div className="space-y-2 text-sm">
                      <div>
                        <span className="font-medium">Custom Name:</span> {persona.custom_name}
                      </div>
                    </div>
                  </div>
                )}

                {persona.custom_instructions && (
                  <div>
                    <h3 className="font-semibold mb-2">Custom Instructions</h3>
                    <div className="text-sm bg-muted p-3 rounded-md">
                      {persona.custom_instructions}
                    </div>
                  </div>
                )}

                <div>
                  <h3 className="font-semibold mb-2">Quota Usage</h3>
                  <div className="space-y-2 text-sm">
                    <div>
                      <span className="font-medium">Kudos Used:</span> {persona.kudos_quota_used.toString()}
                    </div>
                    <div>
                      <span className="font-medium">WTF Used:</span> {persona.wtf_quota_used.toString()}
                    </div>
                    <div>
                      <span className="font-medium">Last Reset:</span> {new Date(persona.last_quota_reset).toLocaleDateString()}
                    </div>
                  </div>
                </div>
              </div>
            </TabsContent>
            
            <TabsContent value="activity" className="p-6">
              <ActivityHistory personaId={persona.id} projectId={persona.project_id} />
            </TabsContent>
            
            <TabsContent value="history" className="p-6">
              <div className="text-center text-muted-foreground">
                <p>Action history tracking coming soon...</p>
              </div>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>

      <PersonaForm
        open={showEditDialog}
        onOpenChange={setShowEditDialog}
        projectId={persona.project_id}
        persona={persona}
        onSuccess={handleUpdate}
      />
    </div>
  );
}