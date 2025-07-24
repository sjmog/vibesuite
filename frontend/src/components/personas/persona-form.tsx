import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { TemplateSelector } from './template-selector';
import { makeRequest } from '@/lib/api';
import type { 
  PersonaTemplate, 
  ProjectPersonaWithTemplate, 
  CreateProjectPersona,
  UpdateProjectPersona,
  ApiResponse 
} from 'shared/types';

interface PersonaFormProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  projectId: string;
  persona?: ProjectPersonaWithTemplate;
  onSuccess: () => void;
}

export function PersonaForm({ 
  open, 
  onOpenChange, 
  projectId, 
  persona, 
  onSuccess 
}: PersonaFormProps) {
  const [selectedTemplate, setSelectedTemplate] = useState<PersonaTemplate | null>(null);
  const [customName, setCustomName] = useState('');
  const [customInstructions, setCustomInstructions] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const isEditing = !!persona;
  const title = isEditing ? 'Edit Persona' : 'Create Persona';
  const description = isEditing 
    ? 'Update the persona configuration' 
    : 'Create a new AI persona for your team';

  useEffect(() => {
    if (persona) {
      setCustomName(persona.custom_name || '');
      setCustomInstructions(persona.custom_instructions || '');
      // For editing, we'll show the current template in the selector
      setSelectedTemplate({
        id: persona.template_id,
        name: persona.template_name,
        role_type: persona.template_role_type,
        description: persona.template_description,
        default_instructions: '',
        capabilities: '',
        tool_restrictions: '',
        automation_triggers: '',
        kudos_quota_daily: BigInt(0),
        is_system: true,
        created_at: '',
        updated_at: ''
      });
    } else {
      setSelectedTemplate(null);
      setCustomName('');
      setCustomInstructions('');
    }
    setError(null);
  }, [persona, open]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!isEditing && !selectedTemplate) {
      setError('Please select a persona template');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      if (isEditing) {
        // Update existing persona
        const updateData: UpdateProjectPersona = {
          custom_name: customName.trim() || null,
          custom_instructions: customInstructions.trim() || null,
          is_active: null,
        };

        const response = await makeRequest(
          `/api/personas/projects/${projectId}/personas/${persona.id}`,
          {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(updateData),
          }
        );

        const data: ApiResponse<any> = await response.json();
        if (!data.success) {
          throw new Error(data.message || 'Failed to update persona');
        }
      } else {
        // Create new persona
        const createData: CreateProjectPersona = {
          project_id: projectId,
          template_id: selectedTemplate!.id,
          custom_name: customName.trim() || null,
          custom_instructions: customInstructions.trim() || null,
        };

        const response = await makeRequest('/api/personas/personas', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(createData),
        });

        const data: ApiResponse<any> = await response.json();
        if (!data.success) {
          throw new Error(data.message || 'Failed to create persona');
        }
      }

      onSuccess();
      onOpenChange(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px]">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>{title}</DialogTitle>
            <DialogDescription>{description}</DialogDescription>
          </DialogHeader>
          
          <div className="space-y-4 py-4">
            {!isEditing && (
              <div className="space-y-2">
                <Label>Persona Template *</Label>
                <TemplateSelector
                  selectedTemplate={selectedTemplate}
                  onTemplateSelect={setSelectedTemplate}
                />
              </div>
            )}

            <div className="space-y-2">
              <Label htmlFor="custom-name">
                Custom Name
                <span className="text-muted-foreground ml-1">(optional)</span>
              </Label>
              <Input
                id="custom-name"
                placeholder={selectedTemplate ? `e.g., Senior ${selectedTemplate.name}` : 'Enter custom name'}
                value={customName}
                onChange={(e) => setCustomName(e.target.value)}
              />
              <p className="text-xs text-muted-foreground">
                Leave empty to use the template name
              </p>
            </div>

            <div className="space-y-2">
              <Label htmlFor="custom-instructions">
                Custom Instructions
                <span className="text-muted-foreground ml-1">(optional)</span>
              </Label>
              <Textarea
                id="custom-instructions"
                placeholder="Additional instructions specific to this project..."
                value={customInstructions}
                onChange={(e) => setCustomInstructions(e.target.value)}
                rows={4}
              />
              <p className="text-xs text-muted-foreground">
                These will be added to the template's default instructions
              </p>
            </div>

            {error && (
              <div className="text-sm text-destructive">
                {error}
              </div>
            )}
          </div>

          <DialogFooter>
            <Button 
              type="button" 
              variant="outline" 
              onClick={() => onOpenChange(false)}
              disabled={loading}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={loading}>
              {loading ? 'Saving...' : (isEditing ? 'Update' : 'Create')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}