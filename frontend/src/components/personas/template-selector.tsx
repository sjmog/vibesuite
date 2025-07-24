import { useState, useEffect } from 'react';
import { Check, ChevronDown, Users } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from '@/components/ui/command';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { Badge } from '@/components/ui/badge';
import { makeRequest } from '@/lib/api';
import type { PersonaTemplate, ApiResponse } from 'shared/types';

interface TemplateSelectorProps {
  selectedTemplate: PersonaTemplate | null;
  onTemplateSelect: (template: PersonaTemplate | null) => void;
}

export function TemplateSelector({ selectedTemplate, onTemplateSelect }: TemplateSelectorProps) {
  const [open, setOpen] = useState(false);
  const [templates, setTemplates] = useState<PersonaTemplate[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchTemplates = async () => {
      try {
        const response = await makeRequest('/api/personas/templates');
        const data: ApiResponse<PersonaTemplate[]> = await response.json();
        
        if (data.success && data.data) {
          setTemplates(data.data);
        }
      } catch (err) {
        console.error('Failed to fetch persona templates:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchTemplates();
  }, []);

  const getRoleDisplayName = (roleType: string) => {
    return roleType.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
  };

  const getSystemTemplates = () => templates.filter(t => t.is_system);
  const getCustomTemplates = () => templates.filter(t => !t.is_system);

  if (loading) {
    return (
      <Button variant="outline" className="w-full justify-between" disabled>
        <span>Loading templates...</span>
        <ChevronDown className="h-4 w-4 opacity-50" />
      </Button>
    );
  }

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-full justify-between"
        >
          {selectedTemplate ? (
            <div className="flex items-center space-x-2">
              <Users className="h-4 w-4" />
              <span>{selectedTemplate.name}</span>
              <Badge variant="secondary" className="text-xs">
                {getRoleDisplayName(selectedTemplate.role_type)}
              </Badge>
            </div>
          ) : (
            "Select a persona template..."
          )}
          <ChevronDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[400px] p-0">
        <Command>
          <CommandInput placeholder="Search templates..." />
          <CommandEmpty>No templates found.</CommandEmpty>
          
          {getSystemTemplates().length > 0 && (
            <CommandGroup heading="System Templates">
              {getSystemTemplates().map((template) => (
                <CommandItem
                  key={template.id}
                  value={`${template.name} ${template.role_type}`}
                  onSelect={() => {
                    onTemplateSelect(template);
                    setOpen(false);
                  }}
                  className="flex items-center space-x-2 p-3"
                >
                  <Check
                    className={`h-4 w-4 ${
                      selectedTemplate?.id === template.id ? "opacity-100" : "opacity-0"
                    }`}
                  />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center space-x-2">
                      <span className="font-medium">{template.name}</span>
                      <Badge variant="outline" className="text-xs">
                        {getRoleDisplayName(template.role_type)}
                      </Badge>
                    </div>
                    <p className="text-sm text-muted-foreground truncate mt-1">
                      {template.description}
                    </p>
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          )}

          {getCustomTemplates().length > 0 && (
            <CommandGroup heading="Custom Templates">
              {getCustomTemplates().map((template) => (
                <CommandItem
                  key={template.id}
                  value={`${template.name} ${template.role_type}`}
                  onSelect={() => {
                    onTemplateSelect(template);
                    setOpen(false);
                  }}
                  className="flex items-center space-x-2 p-3"
                >
                  <Check
                    className={`h-4 w-4 ${
                      selectedTemplate?.id === template.id ? "opacity-100" : "opacity-0"
                    }`}
                  />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center space-x-2">
                      <span className="font-medium">{template.name}</span>
                      <Badge variant="outline" className="text-xs">
                        {getRoleDisplayName(template.role_type)}
                      </Badge>
                    </div>
                    <p className="text-sm text-muted-foreground truncate mt-1">
                      {template.description}
                    </p>
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          )}
        </Command>
      </PopoverContent>
    </Popover>
  );
}